# Production Deploy Story

<StoryHeader
    title="phenotype-nexus Production Deployment"
    :duration="30"
    :gif="'/gifs/phenotype-nexus-production-deploy.gif'"
    difficulty="advanced"
/>

## Objective

Deploy phenotype-nexus in a production environment with high availability,
horizontal scaling, and full observability. This story covers multi-instance
deployment, health monitoring, circuit breakers, and SLO maintenance.

## Context

Your production environment consists of:

| Component | Count | Specification |
|-----------|-------|---------------|
| Kubernetes Nodes | 6 | 8 vCPU, 32GB RAM each |
| API Gateway Pods | 10 | 2 per node |
| Payment Service Pods | 20 | Distributed across nodes |
| Notification Service Pods | 5 | Standby for scale |
| Database | 3 | PostgreSQL primary + 2 replicas |

Current pain points:
- Hardcoded service URLs cause deployment failures
- No automatic failover when instances crash
- Load balancing via kube-proxy has high latency variance
- No visibility into service-to-service calls

## Production Architecture

```
                         ┌─────────────────┐
                         │   Cloud LB      │
                         │ (Route 53/ALB)  │
                         └────────┬────────┘
                                  │
                    ┌─────────────┼─────────────┐
                    ▼             ▼             ▼
              ┌──────────┐ ┌──────────┐ ┌──────────┐
              │   Pod    │ │   Pod    │ │   Pod    │
              │ gateway-1│ │ gateway-2│ │ gateway-3│
              └────┬─────┘ └────┬─────┘ └────┬─────┘
                   │            │            │
                   └────────────┼────────────┘
                                │
                    ┌───────────┼───────────┐
                    ▼           ▼           ▼
              ┌──────────┐ ┌──────────┐ ┌──────────┐
              │  nexus   │ │  nexus   │ │  nexus   │
              │registry-1│ │registry-2│ │registry-3│
              │(standalone│ │(standalone│ │(standalone│
              └──────────┘ └──────────┘ └──────────┘
                                │
                    ┌───────────┼───────────┐
                    ▼           ▼           ▼
              ┌──────────┐ ┌──────────┐ ┌──────────┐
              │ payment-1│ │payment-2│ │payment-3│
              │ (10 pods)│ │ (10 pods)│ │ (10 pods)│
              └──────────┘ └──────────┘ └──────────┘
```

## Implementation

### Step 1: Kubernetes Deployment

```yaml
# k8s/nexus-registry.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: nexus-registry
  labels:
    app: nexus-registry
spec:
  replicas: 3
  selector:
    matchLabels:
      app: nexus-registry
  template:
    metadata:
      labels:
        app: nexus-registry
    spec:
      containers:
        - name: registry
          image: phenotype/nexus:0.2.0
          ports:
            - containerPort: 8080
          env:
            - name: RUST_LOG
              value: "info,nexus=debug"
            - name: HEALTH_CHECK_INTERVAL
              value: "10s"
            - name: TTL_DURATION
              value: "30s"
          resources:
            requests:
              memory: "64Mi"
              cpu: "100m"
            limits:
              memory: "128Mi"
              cpu: "500m"
          livenessProbe:
            httpGet:
              path: /health
              port: 8080
            initialDelaySeconds: 5
            periodSeconds: 10
          readinessProbe:
            httpGet:
              path: /ready
              port: 8080
            initialDelaySeconds: 5
            periodSeconds: 5
---
apiVersion: v1
kind: Service
metadata:
  name: nexus-registry
spec:
  selector:
    app: nexus-registry
  ports:
    - port: 8080
      targetPort: 8080
  type: ClusterIP
```

### Step 2: Service Registration with TTL

```rust
// src/production_registration.rs
use phenotype_nexus::{ServiceRegistry, Service, HealthStatus};
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tracing::{info, warn};

pub struct ServiceRegistration {
    registry: Arc<ServiceRegistry>,
    service_name: String,
    service_address: String,
    ttl: Duration,
}

impl ServiceRegistration {
    pub fn new(
        registry: Arc<ServiceRegistry>,
        service_name: String,
        service_address: String,
    ) -> Self {
        Self {
            registry,
            service_name,
            service_address,
            ttl: Duration::from_secs(30),
        }
    }

    pub async fn start(mut self) {
        info!(service = %self.service_name, "Starting registration");
        
        // Initial registration
        self.register().await;
        
        // TTL refresh loop
        let mut ticker = interval(Duration::from_secs(10));
        loop {
            ticker.tick().await;
            self.register().await;
        }
    }

    async fn register(&self) {
        let service = Service::new_with_tags(
            &self.service_name,
            &self.service_address,
            &["v2", "production"],
        );
        
        // Include TTL in metadata
        let mut meta = std::collections::HashMap::new();
        meta.insert("ttl_secs".to_string(), "30".to_string());
        meta.insert("registered_at".to_string(), 
            chrono::Utc::now().to_rfc3339()
        );
        
        match self.registry.register_with_metadata(service, meta).await {
            Ok(_) => info!(
                service = %self.service_name,
                address = %self.service_address,
                "Service registered"
            ),
            Err(e) => warn!(
                error = %e,
                service = %self.service_name,
                "Registration failed"
            ),
        }
    }
}
```

### Step 3: Circuit Breaker Implementation

```rust
// src/circuit_breaker.rs
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub enum CircuitState {
    Closed,
    Open(Instant),
    HalfOpen,
}

pub struct CircuitBreaker {
    state: Arc<RwLock<CircuitState>>,
    failure_threshold: u64,
    recovery_timeout: Duration,
    failure_count: Arc<AtomicU64>,
    success_count: Arc<AtomicU64>,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u64, recovery_timeout: Duration) -> Self {
        Self {
            state: Arc::new(RwLock::new(CircuitState::Closed)),
            failure_threshold,
            recovery_timeout,
            failure_count: Arc::new(AtomicU64::new(0)),
            success_count: Arc::new(AtomicU64::new(0)),
        }
    }

    pub async fn is_allowed(&self) -> bool {
        let state = self.state.read().await;
        match &*state {
            CircuitState::Closed => true,
            CircuitState::Open(opened_at) => {
                if opened_at.elapsed() >= self.recovery_timeout {
                    false // Will transition to HalfOpen
                } else {
                    false
                }
            }
            CircuitState::HalfOpen => true,
        }
    }

    pub async fn record_success(&self) {
        let mut state = self.state.write().await;
        
        match &*state {
            CircuitState::HalfOpen => {
                self.success_count.fetch_add(1, Ordering::SeqCst);
                if self.success_count.load(Ordering::SeqCst) >= 3 {
                    *state = CircuitState::Closed;
                    self.failure_count.store(0, Ordering::SeqCst);
                    self.success_count.store(0, Ordering::SeqCst);
                }
            }
            CircuitState::Closed => {
                self.failure_count.store(0, Ordering::SeqCst);
            }
            _ => {}
        }
    }

    pub async fn record_failure(&self) {
        let mut state = self.state.write().await;
        
        self.failure_count.fetch_add(1, Ordering::SeqCst);
        
        match &*state {
            CircuitState::HalfOpen => {
                *state = CircuitState::Open(Instant::now());
                self.success_count.store(0, Ordering::SeqCst);
            }
            CircuitState::Closed => {
                if self.failure_count.load(Ordering::SeqCst) >= self.failure_threshold {
                    *state = CircuitState::Open(Instant::now());
                }
            }
            _ => {}
        }
    }

    pub async fn state(&self) -> CircuitState {
        let state = self.state.read().await;
        state.clone()
    }
}
```

### Step 4: Integration with API Gateway

```rust
// src/gateway_with_resilience.rs
use phenotype_nexus::{ServiceRegistry, RoundRobinBalancer, Service};
use crate::circuit_breaker::CircuitBreaker;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::time::Duration;

pub struct ResilientGateway {
    registry: Arc<ServiceRegistry>,
    balancer: RoundRobinBalancer,
    circuit_breakers: HashMap<String, Arc<CircuitBreaker>>,
}

impl ResilientGateway {
    pub fn new(registry: Arc<ServiceRegistry>) -> Self {
        let mut breakers = HashMap::new();
        
        // Create circuit breaker for each service
        for service in ["auth-service", "payment-service", "notification-service"] {
            breakers.insert(
                service.to_string(),
                Arc::new(CircuitBreaker::new(
                    5,  // 5 failures
                    Duration::from_secs(30),  // 30s recovery
                )),
            );
        }
        
        Self {
            registry,
            balancer: RoundRobinBalancer::new(),
            circuit_breakers: breakers,
        }
    }

    pub async fn call_service(
        &self,
        service_name: &str,
        request: Request,
    ) -> Result<Response, Error> {
        let cb = self.circuit_breakers
            .get(service_name)
            .ok_or_else(|| Error::UnknownService(service_name.to_string()))?;
        
        // Check circuit breaker
        if !cb.is_allowed().await {
            return Err(Error::CircuitOpen(service_name.to_string()));
        }
        
        // Discover service
        let services = self.registry
            .discover()
            .name(service_name)
            .tags(&["v2", "production"])
            .await
            .map_err(|e| Error::DiscoveryFailed(e))?;
        
        if services.is_empty() {
            return Err(Error::ServiceUnavailable(service_name.to_string()));
        }
        
        // Select endpoint
        let endpoint = match self.balancer.select(&services).await {
            Ok(ep) => ep,
            Err(e) => {
                cb.record_failure().await;
                return Err(Error::NoEndpoints(e));
            }
        };
        
        // Make request
        match self.forward(&endpoint.address(), request).await {
            Ok(response) => {
                cb.record_success().await;
                Ok(response)
            }
            Err(e) => {
                cb.record_failure().await;
                Err(e)
            }
        }
    }
}
```

### Step 5: Observability Setup

```rust
// src/metrics.rs
use phenotype_nexus::metrics::{histogram, increment, gauge};
use std::time::Instant;

pub struct RequestMetrics {
    start: Instant,
    service_name: String,
    endpoint: String,
}

impl RequestMetrics {
    pub fn new(service_name: String, endpoint: String) -> Self {
        Self {
            start: Instant::now(),
            service_name,
            endpoint,
        }
    }
}

impl Drop for RequestMetrics {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed().as_secs_f64() * 1000.0;
        
        histogram!(
            "nexus.gateway.request.latency",
            elapsed,
            "service" => self.service_name.clone(),
            "endpoint" => self.endpoint.clone(),
        );
        
        increment!(
            "nexus.gateway.request.total",
            "service" => self.service_name.clone(),
            "endpoint" => self.endpoint.clone(),
        );
    }
}

// Expose Prometheus metrics endpoint
pub async fn metrics_handler() -> Response {
    let encoder = prometheus::TextEncoder::new();
    let metric_families = prometheus::gather();
    
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    
    Response::builder()
        .header("Content-Type", "text/plain")
        .body(Body::from(buffer))
        .unwrap()
}
```

## Kubernetes Metrics

```yaml
# k8s/servicemonitor.yaml
apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: nexus-registry
  labels:
    release: prometheus
spec:
  selector:
    matchLabels:
      app: nexus-registry
  endpoints:
    - port: metrics
      path: /metrics
      interval: 15s
```

## Grafana Dashboard Configuration

```json
{
  "dashboard": {
    "title": "phenotype-nexus Production",
    "panels": [
      {
        "title": "Service Discovery Rate",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(nexus_discovery_lookup_total[5m])",
            "legendFormat": "{{service}}"
          }
        ]
      },
      {
        "title": "Discovery Latency P99",
        "type": "graph",
        "targets": [
          {
            "expr": "histogram_quantile(0.99, rate(nexus_discovery_lookup_latency_bucket[5m]))",
            "legendFormat": "P99"
          }
        ]
      },
      {
        "title": "Circuit Breaker States",
        "type": "piechart",
        "targets": [
          {
            "expr": "sum by (state) (nexus_circuit_breaker_state)",
            "legendFormat": "{{state}}"
          }
        ]
      },
      {
        "title": "Error Budget Remaining",
        "type": "gauge",
        "targets": [
          {
            "expr": "100 - (100 * nexus_errors_total / nexus_requests_total)",
            "unit": "percent"
          }
        ]
      }
    ]
  }
}
```

## SLO Configuration

| SLO | Target | Error Budget |
|-----|--------|--------------|
| Discovery Availability | 99.99% | 43.8 min/month |
| Discovery P99 Latency | < 10ms | 0.1% requests |
| Registration Success | 99.95% | 3.65 hr/month |
| Gateway Availability | 99.9% | 43.8 min/month |

## Deployment Checklist

- [ ] Registry deployed with 3 replicas
- [ ] Health checks configured (liveness + readiness)
- [ ] PodDisruptionBudget set for zero downtime
- [ ] Circuit breakers configured per service
- [ ] Metrics exported to Prometheus
- [ ] Grafana dashboard deployed
- [ ] Alerting rules configured
- [ ] Runbook created
- [ ] Load test completed
- [ ] SLO baseline established

## Verification Commands

```bash
# Check registry pods
kubectl get pods -l app=nexus-registry

# View registry logs
kubectl logs -l app=nexus-registry --tail=100

# Check metrics endpoint
curl http://nexus-registry:8080/metrics

# Test discovery via CLI
kubectl exec -it test-pod -- /nexus-cli discover payment-service

# Run load test
kubectl run load-test --image=wrk -- -t10 -c100 -d60s http://api-gateway/checkout
```

## Rollback Procedure

If issues are detected post-deployment:

```bash
# Immediate rollback
kubectl rollout undo deployment/nexus-registry

# Verify rollback
kubectl rollout status deployment/nexus-registry

# Check error rates
kubectl logs -l app=nexus-registry | grep ERROR
```

## Next Steps

- Implement global load balancing across regions
- Add automatic scaling based on discovery rate
- Integrate with service mesh (Linkerd/Istio)
- Enable mTLS for service-to-service communication
