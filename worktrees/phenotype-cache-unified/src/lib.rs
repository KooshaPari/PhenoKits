use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Cache tier configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub max_size: usize,
    pub ttl_seconds: u64,
    pub eviction_policy: EvictionPolicy,
}

#[derive(Debug, Clone, Copy)]
pub enum EvictionPolicy {
    LRU,      // Least Recently Used
    LFU,      // Least Frequently Used
    FIFO,     // First In First Out
    TTL,      // Time To Live based
}

/// Unified multi-tier cache
pub struct UnifiedCache<K, V> {
    l1: Arc<RwLock<HashMap<K, CacheEntry<V>>>>,
    l2: Arc<RwLock<HashMap<K, CacheEntry<V>>>>,
    l3: Arc<RwLock<HashMap<K, CacheEntry<V>>>>,
    config: CacheConfig,
}

#[derive(Debug, Clone)]
pub struct CacheEntry<V> {
    pub value: V,
    pub created_at: std::time::Instant,
    pub last_accessed: std::time::Instant,
    pub access_count: u64,
    pub size_bytes: usize,
}

impl<K: Eq + Hash + Clone, V: Clone> UnifiedCache<K, V> {
    pub fn new(config: CacheConfig) -> Self {
        Self {
            l1: Arc::new(RwLock::new(HashMap::new())),
            l2: Arc::new(RwLock::new(HashMap::new())),
            l3: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Get value from cache
    pub async fn get(&self, key: &K) -> Option<V> {
        // Check L1 first
        if let Some(entry) = self.l1.read().await.get(key) {
            self.update_access(&self.l1, key).await;
            return Some(entry.value.clone());
        }
        
        // Check L2
        if let Some(entry) = self.l2.read().await.get(key) {
            self.update_access(&self.l1, key).await;
            self.l2.write().await.remove(key);
            return Some(entry.value.clone());
        }
        
        // Check L3 (persistent)
        if let Some(entry) = self.l3.read().await.get(key) {
            self.update_access(&self.l1, key).await;
            self.l3.write().await.remove(key);
            return Some(entry.value.clone());
        }
        
        None
    }

    /// Set value in cache
    pub async fn set(&self, key: K, value: V, size_bytes: usize) {
        let entry = CacheEntry {
            value,
            created_at: std::time::Instant::now(),
            last_accessed: std::time::Instant::now(),
            access_count: 1,
            size_bytes,
        };

        // Add to L1
        self.l1.write().await.insert(key, entry);
        
        // Evict if needed
        self.evict_if_needed().await;
    }

    async fn update_access(&self, tier: &Arc<RwLock<HashMap<K, CacheEntry<V>>>>, key: &K) {
        if let Some(entry) = tier.write().await.get_mut(key) {
            entry.last_accessed = std::time::Instant::now();
            entry.access_count += 1;
        }
    }

    async fn evict_if_needed(&self) {
        let l1_size = self.l1.read().await.len();
        if l1_size > self.config.max_size {
            self.evict_l1().await;
        }
    }

    async fn evict_l1(&self) {
        let mut tier = self.l1.write().await;
        match self.config.eviction_policy {
            EvictionPolicy::LRU => {
                if let Some((key, _)) = tier.iter()
                    .min_by_key(|(_, e)| e.last_accessed)
                    .map(|(k, v)| (k.clone(), v.clone()))
                {
                    tier.remove(&key);
                }
            }
            EvictionPolicy::LFU => {
                if let Some((key, _)) = tier.iter()
                    .min_by_key(|(_, e)| e.access_count)
                    .map(|(k, v)| (k.clone(), v.clone()))
                {
                    tier.remove(&key);
                }
            }
            _ => {
                if let Some((key, _)) = tier.iter()
                    .min_by_key(|(_, e)| e.created_at)
                    .map(|(k, v)| (k.clone(), v.clone()))
                {
                    tier.remove(&key);
                }
            }
        }
    }

    /// Clear all cache tiers
    pub async fn clear(&self) {
        self.l1.write().await.clear();
        self.l2.write().await.clear();
        self.l3.write().await.clear();
    }

    /// Get cache statistics
    pub async fn stats(&self) -> CacheStats {
        CacheStats {
            l1_size: self.l1.read().await.len(),
            l2_size: self.l2.read().await.len(),
            l3_size: self.l3.read().await.len(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub l1_size: usize,
    pub l2_size: usize,
    pub l3_size: usize,
}
