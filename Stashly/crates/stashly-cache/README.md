# stashly-cache

Two-tier LRU cache with DashMap and TTL support.

**Features:**
- Layer 1: In-memory LRU cache (bounded size)
- Layer 2: DashMap concurrent hash map
- Automatic TTL expiration
- Async-safe operations

**Tests:** Inline `#[test]` in `src/lib.rs`

**Usage:**
```rust
let cache = TwoTierCache::new(1000);
cache.insert("key", "value");
assert_eq!(cache.get("key"), Some("value".to_string()));
```
