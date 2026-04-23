# Health Checks: State of the Art Research

> **HelMo Research Document**  
> **Topic:** Health Check Patterns, Probe Implementation & Reliability Engineering  
> **Date:** April 2025  
> **Status:** Comprehensive SOTA Analysis

---

## Executive Summary

Health checks form the foundational layer of modern distributed systems observability. This research document examines contemporary health check patterns, from basic liveness probes to sophisticated deep health diagnostics, with particular focus on Kubernetes-native implementations and cloud-native architectures. The analysis covers probe types, implementation strategies, anti-patterns, and emerging standards in health monitoring.

---

## Table of Contents

1. [Fundamental Concepts](#1-fundamental-concepts)
2. [Probe Types Deep Dive](#2-probe-types-deep-dive)
3. [Deep vs Shallow Health Checks](#3-deep-vs-shallow-health-checks)
4. [Kubernetes Health Model](#4-kubernetes-health-model)
5. [Implementation Patterns](#5-implementation-patterns)
6. [SLI/SLO Integration](#6-slislo-integration)
7. [Anti-Patterns & Pitfalls](#7-anti-patterns--pitfalls)
8. [Emerging Standards](#8-emerging-standards)
9. [References](#9-references)

---

## 1. Fundamental Concepts

### 1.1 The Health Check Philosophy

Health checks represent the fundamental mechanism by which orchestrators, load balancers, and monitoring systems determine whether a service is functioning correctly. At their core, health checks answer a deceptively simple question: **"Is this system capable of serving traffic?"**

The evolution of health checking mirrors the evolution of distributed systems themselves:

| Era | Characteristic | Health Check Approach |
|-----|-----------------|----------------------|
| Monolithic | Single binary, single host | Simple process monitoring |
| SOA | Multiple services, dedicated hosts | Port-based TCP checks |
| Microservices | Many services, shared infrastructure | HTTP endpoint checks |
| Cloud-Native | Containerized, orchestrated | Multi-dimensional probe system |
| Serverless | Function-as-a-service | Implicit platform health |

### 1.2 The CAP Theorem Implications

Health checking exists at the intersection of Consistency, Availability, and Partition tolerance. The design of health check endpoints must consider:

- **Consistency**: Should the health check return the same result from all replicas?
- **Availability**: Can the health check endpoint itself remain available during partial failures?
- **Partition Tolerance**: How does the system behave when health check signals are delayed or lost?

### 1.3 Health State Machine

Modern health systems implement a state machine with typically three states:

```
        +-----------+     failure      +-----------+
        |  HEALTHY  |<----------------| UNHEALTHY |
        +-----+-----+                 +-----+-----+
              |                             |
              |       +-----------+         |
              +------>|  UNKNOWN  |<--------+
                      +-----------+
                      (initial/transition)
```

**State Transitions:**
- `UNKNOWN → HEALTHY`: Successful probe threshold reached
- `HEALTHY → UNHEALTHY`: Failure threshold exceeded
- `UNHEALTHY → HEALTHY`: Recovery threshold reached
- Any state → `UNKNOWN`: Probe timeout or no data

### 1.4 Health Check Topology

The topology of health checking creates distinct patterns:

#### Push Model
- Agent reports health to central collector
- Examples: StatsD, custom heartbeat
- Pros: Works through NAT/firewalls, lower latency detection
- Cons: Requires agent deployment, harder to verify agent health

#### Pull Model
- Monitor polls service endpoint
- Examples: Prometheus scraping, HTTP probes
- Pros: Simpler service implementation, monitor controls frequency
- Cons: Requires network accessibility, potential thundering herd

#### Passive Model
- Health inferred from observed traffic
- Examples: Load balancer health from request success rates
- Pros: No artificial probe traffic, reflects actual user experience
- Cons: Requires traffic to detect issues, slower detection

---

## 2. Probe Types Deep Dive

### 2.1 Liveness Probes

**Purpose:** Determine if the application process is running and not deadlocked.

**Kubernetes Implementation:**

```yaml
apiVersion: v1
kind: Pod
spec:
  containers:
  - name: app
    livenessProbe:
      httpGet:
        path: /health/live
        port: 8080
      initialDelaySeconds: 10
      periodSeconds: 5
      timeoutSeconds: 2
      failureThreshold: 3
      successThreshold: 1
```

**Behavioral Semantics:**
- **FAILURE**: Kubernetes restarts the container
- **SUCCESS**: No action taken
- **Purpose**: Recover from deadlocks, infinite loops, resource exhaustion

**Design Principles for Liveness:**

1. **Minimal Dependencies**: Liveness checks should verify the process is alive, not external dependencies
2. **Fast Response**: Should complete in milliseconds
3. **Lightweight**: No database queries, no external calls
4. **Local Scope**: Only internal state validation

**Recommended Liveness Endpoint Implementation:**

```python
# Python FastAPI example
@app.get("/health/live")
async def liveness():
    """
    Liveness probe - returns 200 if the process is running.
    
    This endpoint must:
    - Complete quickly (<100ms)
    - Not check external dependencies
    - Not allocate significant resources
    """
    return {"status": "alive", "timestamp": time.time()}
```

```rust
// Rust Actix-web example
#[get("/health/live")]
async fn liveness() -> impl Responder {
    // Minimal check - just return OK
    // No database, no cache, no external calls
    HttpResponse::Ok().json(json!({
        "status": "alive",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
```

### 2.2 Readiness Probes

**Purpose:** Determine if the application is ready to receive traffic.

**Kubernetes Implementation:**

```yaml
apiVersion: v1
kind: Pod
spec:
  containers:
  - name: app
    readinessProbe:
      httpGet:
        path: /health/ready
        port: 8080
      initialDelaySeconds: 5
      periodSeconds: 3
      timeoutSeconds: 3
      failureThreshold: 2
      successThreshold: 1
```

**Behavioral Semantics:**
- **FAILURE**: Pod removed from Service endpoints (no traffic)
- **SUCCESS**: Pod added to Service endpoints (receives traffic)
- **Purpose**: Prevent traffic to initializing or recovering instances

**Readiness Check Components:**

```
┌─────────────────────────────────────────┐
│         READINESS EVALUATION            │
├─────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐    │
│  │  App Init    │  │ Dependencies │    │
│  │   Complete   │  │   Healthy    │    │
│  └──────┬───────┘  └──────┬───────┘    │
│         │                 │            │
│         └────────┬────────┘            │
│                  ▼                      │
│         ┌──────────────┐               │
│         │   Circuit    │               │
│         │   Breakers   │               │
│         └──────┬───────┘               │
│                ▼                       │
│         ┌──────────────┐               │
│         │ Load/Stress  │               │
│         │   Within     │               │
│         │   Limits     │               │
│         └──────┬───────┘               │
│                ▼                       │
│           ┌─────────┐                  │
│           │  READY  │                  │
│           └─────────┘                  │
└─────────────────────────────────────────┘
```

**Comprehensive Readiness Implementation:**

```python
@app.get("/health/ready")
async def readiness():
    """
    Readiness probe - comprehensive dependency check.
    
    Checks:
    1. Database connectivity
    2. Cache availability
    3. Message queue connection
    4. Critical external services
    """
    checks = {
        "database": await check_database(),
        "cache": await check_cache(),
        "queue": await check_message_queue(),
        "external_api": await check_external_api(),
    }
    
    all_healthy = all(c["healthy"] for c in checks.values())
    
    if all_healthy:
        return JSONResponse(
            content={"status": "ready", "checks": checks},
            status_code=200
        )
    else:
        return JSONResponse(
            content={"status": "not_ready", "checks": checks},
            status_code=503  # Service Unavailable
        )
```

### 2.3 Startup Probes

**Purpose:** Determine when a slow-starting application has completed initialization.

**The Problem:** Long-initializing applications may fail liveness checks before fully started, causing premature restarts.

**Kubernetes Implementation:**

```yaml
apiVersion: v1
kind: Pod
spec:
  containers:
  - name: app
    startupProbe:
      httpGet:
        path: /health/startup
        port: 8080
      initialDelaySeconds: 5
      periodSeconds: 5
      timeoutSeconds: 3
      failureThreshold: 30  # 30 * 5s = 150s max startup time
    livenessProbe:
      httpGet:
        path: /health/live
        port: 8080
      periodSeconds: 10
      failureThreshold: 3
      # Liveness disabled until startup succeeds
```

**Startup vs Liveness/Readiness Interaction:**

```
Timeline:
├─ Container Starts ─┬───────────────────────────────────────────────────┬─
                     │                                                   │
                     ▼                                                   ▼
              ┌─────────────┐                                   ┌─────────────┐
              │   STARTUP   │ ──success──▶ ┌─────────────┐     │   NORMAL    │
              │    PROBE    │              │   LIVENESS  │ ──▶  │   OPERATION │
              │   RUNNING   │              │   ENABLED   │      │             │
              └─────────────┘              └─────────────┘      └─────────────┘
                    │
              failureThreshold
              exceeded
                    │
                    ▼
              ┌─────────────┐
              │  CONTAINER  │
              │  RESTART    │
              └─────────────┘
```

**Startup Check Implementation:**

```python
@app.get("/health/startup")
async def startup():
    """
    Startup probe - verifies application initialization complete.
    
    Checks:
    1. Configuration loaded
    2. Migrations completed
    3. Warmup operations done
    4. Initial data cached
    """
    startup_checks = {
        "config_loaded": config.is_loaded(),
        "migrations_complete": await migrations.status(),
        "cache_warmed": cache.warmup_complete(),
        "connections_established": connections.ready(),
    }
    
    if all(startup_checks.values()):
        return {"status": "started", "checks": startup_checks}
    else:
        return JSONResponse(
            content={"status": "starting", "checks": startup_checks},
            status_code=503
        )
```

### 2.4 Probe Configuration Best Practices

**Timing Parameters Reference:**

| Parameter | Typical Range | Guidance |
|-----------|---------------|----------|
| `periodSeconds` | 5-30s | Balance detection speed vs. overhead |
| `timeoutSeconds` | 1-5s | Must be < periodSeconds |
| `failureThreshold` | 2-5 | Consider transient network issues |
| `successThreshold` | 1-3 | Higher for stateful services |
| `initialDelaySeconds` | 0-60s | Only for known slow starts |

**Probe Protocol Selection:**

| Protocol | Use Case | Pros | Cons |
|----------|----------|------|------|
| HTTP GET | Web services, REST APIs | Rich response, headers | Requires HTTP server |
| TCP Socket | Databases, message queues | Fast, minimal overhead | Limited diagnostics |
| gRPC | gRPC services | Native protocol support | Requires gRPC health proto |
| Exec | Legacy apps, custom checks | Arbitrary logic | Resource intensive |

---

## 3. Deep vs Shallow Health Checks

### 3.1 The Depth Spectrum

Health checks exist on a spectrum of depth:

```
SHALLOW ◄─────────────────────────────────────────► DEEP
    │                                               │
    ├─ Process running                              ├─ Full business transaction
    ├─ HTTP responding                              ├─ End-to-end test
    ├─ Dependencies reachable                       ├─ Synthetic user journey
    ├─ Database connectable                         ├─ Cross-region validation
    └─ Basic query works                            └─ Load test integration
```

### 3.2 Shallow Health Checks

**Characteristics:**
- Response time: < 50ms
- Resource usage: Minimal
- Dependency scope: None or localhost only
- Accuracy: May miss functional issues

**Implementation Pattern:**

```python
class ShallowHealthChecker:
    """Minimal health verification for high-frequency checks."""
    
    async def check(self) -> HealthStatus:
        # Check 1: Process can respond
        if not self._response_channel_healthy():
            return HealthStatus.DEGRADED
        
        # Check 2: Internal state valid
        if self._state_corrupted():
            return HealthStatus.UNHEALTHY
        
        return HealthStatus.HEALTHY
```

**Appropriate For:**
- Liveness probes (Kubernetes)
- High-frequency load balancer checks
- Canaries in large fleets

### 3.3 Deep Health Checks

**Characteristics:**
- Response time: 100ms - 5s
- Resource usage: Moderate to high
- Dependency scope: All critical dependencies
- Accuracy: High functional verification

**Implementation Pattern:**

```python
class DeepHealthChecker:
    """Comprehensive health verification for critical decisions."""
    
    async def check(self) -> DeepHealthStatus:
        results = await asyncio.gather(
            self._check_database_transactions(),
            self._check_cache_read_write(),
            self._check_external_api_endpoints(),
            self._check_message_queue_publish(),
            self._check_file_system_operations(),
            return_exceptions=True
        )
        
        return self._aggregate_results(results)
```

**Appropriate For:**
- Readiness probes (Kubernetes)
- Deployment gate decisions
- Critical path monitoring

### 3.4 The Cascading Check Pattern

A sophisticated approach combining both depths:

```python
class CascadingHealthChecker:
    """
    Implements tiered health checking.
    
    Level 1: Shallow (always run)
    Level 2: Standard (run if shallow passes)
    Level 3: Deep (run periodically, not every request)
    """
    
    def __init__(self):
        self._deep_check_interval = 30  # seconds
        self._last_deep_check = 0
        self._cached_deep_result = None
    
    async def health(self, requested_depth: Depth) -> HealthReport:
        # Level 1: Always run shallow
        shallow = await self._shallow_check()
        if not shallow.healthy:
            return HealthReport(level=1, status=shallow)
        
        if requested_depth == Depth.SHALLOW:
            return HealthReport(level=1, status=shallow)
        
        # Level 2: Standard dependency checks
        standard = await self._standard_check()
        if not standard.healthy:
            return HealthReport(level=2, status=standard)
        
        if requested_depth == Depth.STANDARD:
            return HealthReport(level=2, status=standard)
        
        # Level 3: Deep check (with caching)
        deep = await self._get_deep_check()
        return HealthReport(level=3, status=deep)
    
    async def _get_deep_check(self) -> HealthStatus:
        now = time.time()
        if now - self._last_deep_check > self._deep_check_interval:
            self._cached_deep_result = await self._deep_check()
            self._last_deep_check = now
        return self._cached_deep_result
```

### 3.5 Dependency Depth Matrix

| Dependency | Shallow Check | Deep Check |
|------------|---------------|------------|
| Database | TCP connection established | Execute test transaction, verify replication lag |
| Cache | Socket connection open | Read/write operation, verify hit ratio |
| Message Queue | Connection active | Publish and consume test message |
| External API | DNS resolves, TCP reachable | Full request/response with validation |
| File System | Directory exists | Write, sync, read, delete test file |

---

## 4. Kubernetes Health Model

### 4.1 The Kubernetes Probe Architecture

Kubernetes implements a comprehensive health model through its kubelet agent:

```
┌─────────────────────────────────────────────────────────────┐
│                         KUBELET                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐           │
│  │  Liveness   │  │  Readiness  │  │   Startup   │           │
│  │   Probe     │  │   Probe     │  │   Probe     │           │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘           │
│         │                │                │                  │
│         └────────────────┴────────────────┘                │
│                          │                                   │
│                    ┌─────┴─────┐                            │
│                    │ Container │                            │
│                    └─────┬─────┘                            │
│                          │                                   │
│              ┌───────────┼───────────┐                      │
│              ▼           ▼           ▼                      │
│         ┌────────┐  ┌────────┐  ┌────────┐                 │
│         │Restart │  │Endpoint│  │Startup │                 │
│         │Policy  │  │ Removal│  │ Gate   │                 │
│         └────────┘  └────────┘  └────────┘                 │
└─────────────────────────────────────────────────────────────┘
```

### 4.2 Pod Lifecycle Integration

The probe system integrates with the Pod lifecycle state machine:

```
         Pending
            │
            ▼ (containers scheduled)
┌───────────────────────┐
│    ContainerCreating  │
└───────────┬───────────┘
            │
            ▼ (startup probe begins)
┌───────────────────────┐
│       Running          │◄─────────────────────────────┐
│  (startup probe running)│                              │
└───────────┬───────────┘                              │
            │                                            │
    ┌───────┴───────┐                                    │
    ▼               ▼                                    │
┌────────┐    ┌────────┐                                 │
│ Success│    │ Failure│ ──► Container Restart ──────────┘
└───┬────┘    └────────┘
    │
    ▼ (liveness/readiness enabled)
┌───────────────────────┐
│       Running          │
│ (normal operation)     │◄─────────────────────────────┐
└───────────┬───────────┘                              │
            │                                          │
    ┌───────┴───────┐                                  │
    ▼               ▼                                  │
┌────────┐    ┌────────┐                               │
│ Succeeded│   │ Failed │ ──► Container Restart ───────┘
└────────┘    └────────┘
```

### 4.3 Probe Handler Types

**HTTP GET Handler:**

```yaml
livenessProbe:
  httpGet:
    path: /healthz
    port: 8080
    httpHeaders:
    - name: Accept
      value: application/json
    - name: X-Health-Check
      value: kubelet
  periodSeconds: 10
```

**TCP Socket Handler:**

```yaml
livenessProbe:
  tcpSocket:
    port: 3306
  initialDelaySeconds: 15
  periodSeconds: 20
```

**gRPC Handler (requires gRPC Health Checking Protocol):**

```yaml
livenessProbe:
  grpc:
    port: 50051
    # service: "" # empty for overall health
  periodSeconds: 10
```

**Exec Handler:**

```yaml
livenessProbe:
  exec:
    command:
    - /bin/sh
    - -c
    - "pgrep myprocess && test -f /var/run/healthy"
  periodSeconds: 10
```

### 4.4 Advanced Kubernetes Health Patterns

**Pre-Stop Hook Integration:**

```yaml
lifecycle:
  preStop:
    exec:
      command: ["/bin/sh", "-c", "sleep 15 && curl -X POST localhost:8080/drain"]
livenessProbe:
  httpGet:
    path: /health
    port: 8080
  periodSeconds: 10
readinessProbe:
  httpGet:
    path: /ready
    port: 8080
  periodSeconds: 5
```

**Sidecar Health Coordination:**

When running multiple containers in a Pod, health checks must coordinate:

```yaml
# Main application container
- name: app
  readinessProbe:
    httpGet:
      path: /health
      port: 8080
  lifecycle:
    preStop:
      exec:
        command: ["sh", "-c", "touch /tmp/shutting-down && sleep 30"]

# Sidecar container
- name: proxy
  readinessProbe:
    exec:
      command: ["sh", "-c", "test ! -f /shared/shutting-down && curl localhost:15000/ready"]
```

### 4.5 Pod Disruption Budgets and Health

Health checks interact with Pod Disruption Budgets (PDB):

```yaml
apiVersion: policy/v1
kind: PodDisruptionBudget
metadata:
  name: app-pdb
spec:
  minAvailable: 2
  selector:
    matchLabels:
      app: critical-service
```

**Eviction Handling:**
- During voluntary disruptions (node drain, upgrades), Kubernetes respects PDB
- Unhealthy pods (failing readiness) don't count toward PDB requirements
- Pre-stop hooks allow graceful handling of eviction signals

---

## 5. Implementation Patterns

### 5.1 The Health Aggregator Pattern

For services with multiple subsystems:

```python
@dataclass
class HealthComponent:
    name: str
    healthy: bool
    latency_ms: float
    last_check: datetime
    metadata: Dict[str, Any]

class HealthAggregator:
    """Aggregates health from multiple components."""
    
    def __init__(self):
        self._components: Dict[str, HealthChecker] = {}
        self._weights: Dict[str, float] = {}
    
    def register(self, name: str, checker: HealthChecker, weight: float = 1.0):
        self._components[name] = checker
        self._weights[name] = weight
    
    async def check_all(self) -> AggregatedHealth:
        tasks = [
            self._check_component(name, checker)
            for name, checker in self._components.items()
        ]
        results = await asyncio.gather(*tasks, return_exceptions=True)
        
        components = {}
        for (name, _), result in zip(self._components.items(), results):
            if isinstance(result, Exception):
                components[name] = HealthComponent(
                    name=name,
                    healthy=False,
                    latency_ms=0,
                    last_check=datetime.utcnow(),
                    metadata={"error": str(result)}
                )
            else:
                components[name] = result
        
        overall = self._calculate_overall(components)
        return AggregatedHealth(overall=overall, components=components)
    
    def _calculate_overall(self, components: Dict[str, HealthComponent]) -> HealthStatus:
        total_weight = sum(self._weights.values())
        healthy_weight = sum(
            self._weights[name] for name, c in components.items() if c.healthy
        )
        
        ratio = healthy_weight / total_weight
        
        if ratio == 1.0:
            return HealthStatus.HEALTHY
        elif ratio >= 0.5:
            return HealthStatus.DEGRADED
        else:
            return HealthStatus.UNHEALTHY
```

### 5.2 Circuit Breaker Integration

Health checks should integrate with circuit breakers:

```python
class CircuitBreakerHealthAdapter:
    """Adapts circuit breaker state to health status."""
    
    def __init__(self, circuit_breaker: CircuitBreaker):
        self._cb = circuit_breaker
    
    @property
    def healthy(self) -> bool:
        return self._cb.state != CircuitState.OPEN
    
    @property
    def status(self) -> str:
        return {
            CircuitState.CLOSED: "healthy",
            CircuitState.HALF_OPEN: "degraded",
            CircuitState.OPEN: "unhealthy",
        }[self._cb.state]
```

### 5.3 Health Check Caching

To prevent thundering herd on dependency checks:

```python
class CachedHealthChecker:
    """Caches health check results with TTL."""
    
    def __init__(
        self,
        checker: HealthChecker,
        ttl_seconds: float = 5.0,
        stale_while_revalidate: bool = True
    ):
        self._checker = checker
        self._ttl = ttl_seconds
        self._stale_while_revalidate = stale_while_revalidate
        self._cache: Optional[HealthResult] = None
        self._last_check = 0.0
        self._lock = asyncio.Lock()
    
    async def check(self) -> HealthResult:
        now = time.time()
        
        # Fast path: cache valid
        if self._cache and now - self._last_check < self._ttl:
            return self._cache
        
        async with self._lock:
            # Double-check after acquiring lock
            if self._cache and now - self._last_check < self._ttl:
                return self._cache
            
            try:
                result = await self._checker.check()
                self._cache = result
                self._last_check = now
                return result
            except Exception as e:
                if self._stale_while_revalidate and self._cache:
                    # Return stale result on error
                    return self._cache
                raise
```

### 5.4 Observability Integration

Health checks should emit metrics:

```python
class InstrumentedHealthChecker:
    """Wraps a health checker with metrics emission."""
    
    def __init__(self, checker: HealthChecker, metrics: MetricsCollector):
        self._checker = checker
        self._metrics = metrics
    
    async def check(self) -> HealthResult:
        start = time.time()
        
        try:
            result = await self._checker.check()
            duration = time.time() - start
            
            # Record metrics
            self._metrics.histogram("health.check.duration", duration)
            self._metrics.gauge("health.status", 1 if result.healthy else 0)
            self._metrics.counter("health.check.total").inc()
            
            if not result.healthy:
                self._metrics.counter("health.check.failures").inc()
            
            return result
            
        except Exception as e:
            duration = time.time() - start
            self._metrics.histogram("health.check.duration", duration)
            self._metrics.counter("health.check.errors").inc()
            raise
```

---

## 6. SLI/SLO Integration

### 6.1 Service Level Indicators (SLIs)

Health checks inform SLI calculation:

| SLI Category | Health Check Input | Calculation |
|--------------|-------------------|-------------|
| Availability | Readiness probe success rate | `healthy_requests / total_requests` |
| Latency | Health check response times | Percentile distribution |
| Error Rate | Failed health checks | `failed_checks / total_checks` |
| Saturation | Resource check metrics | CPU/memory/disk thresholds |

### 6.2 SLO-Driven Health Design

SLOs should drive health check design:

```
SLO: 99.9% availability over 30 days

Health Check Implications:
├── Detection Time: Must be < (1 - 0.999) * 30 days = 43.2 minutes/month
├── Probe Frequency: At least every 10s for rapid detection
├── Failure Threshold: Account for transient failures (3-5 consecutive)
└── Alerting: Page when error budget consumption rate exceeds threshold
```

### 6.3 Error Budget Integration

```python
class ErrorBudgetHealthGate:
    """
    Modifies health threshold based on remaining error budget.
    
    When error budget is healthy: Strict health checks
    When error budget is depleted: More lenient checks to prevent cascading
    """
    
    def __init__(self, error_budget: ErrorBudgetTracker):
        self._budget = error_budget
    
    def get_health_threshold(self) -> float:
        budget_remaining = self._budget.remaining_ratio()
        
        # As budget depletes, require more consecutive failures
        # before marking unhealthy (prevents cascading failures)
        thresholds = {
            (0.75, 1.0): 2,    # Healthy budget: 2 failures
            (0.50, 0.75): 3,   # Warning: 3 failures
            (0.25, 0.50): 5,   # Critical: 5 failures
            (0.0, 0.25): 10,   # Depleted: 10 failures (very lenient)
        }
        
        for (low, high), threshold in thresholds.items():
            if low <= budget_remaining <= high:
                return threshold
        
        return 10
```

---

## 7. Anti-Patterns & Pitfalls

### 7.1 Common Anti-Patterns

**1. The "Everything Check" Liveness Probe**

```python
# ANTI-PATTERN: Liveness checking external dependencies
@app.get("/health/live")  # This is WRONG
def liveness():
    # Don't do this! Liveness should be simple.
    db.ping()
    cache.get("test")
    external_api.status()
    return {"status": "ok"}
```

**2. The Synchronous Block**

```python
# ANTI-PATTERN: Blocking the health endpoint
@app.get("/health")
def health():
    # Synchronous check of slow dependencies
    for service in all_services:  # Don't iterate synchronously
        service.check()  # This blocks!
    return {"status": "ok"}
```

**3. The Uncaught Exception**

```python
# ANTI-PATTERN: Unhandled exceptions
@app.get("/health")
def health():
    # Exception escapes = 500 error
    # Some load balancers treat 500 as "unhealthy"
    # Others might treat it as "don't change state"
    database.query("SELECT 1")  # Might raise!
    return {"status": "ok"}
```

**4. The Cache Poisoning**

```python
# ANTI-PATTERN: Caching failures too long
_cache = {"status": "ok", "timestamp": 0}

@app.get("/health")
def health():
    if time.time() - _cache["timestamp"] < 300:  # 5 minutes is too long!
        return _cache
    # ... expensive check
```

### 7.2 Kubernetes-Specific Pitfalls

| Pitfall | Symptom | Solution |
|---------|---------|----------|
| Initial delay too short | CrashLoopBackOff during startup | Use startup probe |
| Period too aggressive | High CPU from health checks | Increase periodSeconds |
| Timeout too long | Slow failure detection | Set timeout < period |
| Failure threshold too low | Unnecessary restarts | Increase to 3-5 |
| Missing preStop | Connection drops during shutdown | Implement graceful shutdown |

### 7.3 Security Considerations

Health endpoints can leak information:

```python
# ANTI-PATTERN: Information leakage
@app.get("/health")
def health():
    return {
        "database": {
            "host": "internal-db-01.prod.cluster.local",  # Leaks topology!
            "password": "****",  # Still shows there IS a password
            "version": "PostgreSQL 14.2"  # Leaks version for exploits
        },
        "internal_ips": ["10.0.1.45", "10.0.1.46"],  # Leaks network
    }
```

**Best Practice:** Sanitize health check responses:

```python
@app.get("/health")
def health():
    return {
        "status": "healthy",
        "checks": {
            "database": "pass",  # Just pass/fail
            "cache": "pass",
        }
    }
```

---

## 8. Emerging Standards

### 8.1 gRPC Health Checking Protocol

The gRPC ecosystem has standardized health checking:

```protobuf
syntax = "proto3";

package grpc.health.v1;

message HealthCheckRequest {
  string service = 1;
}

message HealthCheckResponse {
  enum ServingStatus {
    UNKNOWN = 0;
    SERVING = 1;
    NOT_SERVING = 2;
    SERVICE_UNKNOWN = 3;
  }
  ServingStatus status = 1;
}

service Health {
  rpc Check(HealthCheckRequest) returns (HealthCheckResponse);
  rpc Watch(HealthCheckRequest) returns (stream HealthCheckResponse);
}
```

**Advantages:**
- Streaming updates (Watch RPC) for real-time health
- Per-service health (not just process-level)
- Language-agnostic standard

### 8.2 OpenTelemetry Health Signals

OpenTelemetry is extending to health signals:

```yaml
# Proposed OTel health signal format
health.signal:
  timestamp: 2025-04-05T12:00:00Z
  resource:
    service.name: payment-service
    service.instance.id: pod-abc-123
  health:
    status: healthy  # healthy, degraded, unhealthy
    components:
      - name: database
        status: healthy
        latency_ms: 12
      - name: cache
        status: degraded
        latency_ms: 150
        reason: "elevated_latency"
```

### 8.3 IETF Health Check API Draft

The IETF is working on a standard health check format:

```json
{
  "status": "warn",
  "version": "1",
  "releaseId": "1.2.3",
  "notes": ["Partitioned from downstream database"],
  "output": "",
  "checks": {
    "datastore:latency": [
      {
        "componentId": "postgres-primary",
        "componentType": "datastore",
        "observedValue": 250,
        "observedUnit": "ms",
        "status": "warn",
        "time": "2025-04-05T12:00:00Z"
      }
    ]
  },
  "links": {
    "about": "https://docs.example.com/health"
  }
}
```

---

## 9. References

### 9.1 Official Documentation

1. **Kubernetes Documentation - Configure Liveness, Readiness and Startup Probes**
   - URL: https://kubernetes.io/docs/tasks/configure-pod-container/configure-liveness-readiness-startup-probes/
   - Author: Kubernetes Authors
   - Description: Official Kubernetes probe configuration guide

2. **gRPC Health Checking Protocol**
   - URL: https://github.com/grpc/grpc/blob/master/doc/health-checking.md
   - Author: gRPC Authors
   - Description: gRPC standard health checking protocol specification

3. **IETF Health Check API Draft**
   - URL: https://datatracker.ietf.org/doc/html/draft-inadarei-api-health-check
   - Author: Irakli Nadareishvili
   - Description: Proposed standard for health check API format

### 9.2 Research Papers

4. **"Fail at Scale: Reliability in the Face of Rapid Change"**
   - Authors: Benjamin Treynor Sloss et al.
   - Conference: USENIX LISA 2014
   - URL: https://www.usenix.org/conference/lisa14/conference-program/presentation/sloss
   - Description: Google's approach to health monitoring at scale

5. **"Microservices: A Definition of This New Architectural Term"**
   - Author: Martin Fowler
   - URL: https://martinfowler.com/articles/microservices.html
   - Description: Foundational microservices concepts including health patterns

### 9.3 Industry Best Practices

6. **Site Reliability Engineering (SRE Book)**
   - Authors: Betsy Beyer, Chris Jones, Jennifer Petoff, Niall Richard Murphy
   - Publisher: O'Reilly Media
   - URL: https://sre.google/sre-book/table-of-contents/
   - Chapters: Monitoring Distributed Systems, Eliminating Toil

7. **The Twelve-Factor App - Admin Processes**
   - URL: https://12factor.net/admin-processes
   - Description: Health checks as admin processes

### 9.4 Tool-Specific Resources

8. **Prometheus - Writing Exporters**
   - URL: https://prometheus.io/docs/instrumenting/writing_exporters/
   - Description: Best practices for exporter health

9. **AWS Well-Architected - Health Dashboard Pattern**
   - URL: https://docs.aws.amazon.com/wellarchitected/latest/operational-excellence-pillar/health-dashboard.html
   - Description: AWS health dashboard architecture

### 9.5 Open Source Implementations

10. **Spring Boot Actuator**
    - URL: https://docs.spring.io/spring-boot/docs/current/reference/html/actuator.html
    - Description: Comprehensive health check framework for JVM

11. **Dropwizard Metrics - Health Checks**
    - URL: https://metrics.dropwizard.io/4.2.0/manual/healthchecks.html
    - Description: Health check library for Java applications

12. **Go micro/go-plugins - Health**
    - URL: https://github.com/micro/go-plugins/tree/master/health
    - Description: Go microservices health check implementations

---

## Appendix A: Quick Reference Card

### Probe Configuration Matrix

| Scenario | Liveness | Readiness | Startup |
|----------|----------|-----------|---------|
| Fast-starting API | period: 10s, timeout: 1s | period: 5s, timeout: 3s | Not needed |
| Database-backed service | Simple response | DB connectivity check | period: 5s, failureThreshold: 30 |
| Message consumer | Process check | Queue connection + lag | Warmup check |
| Frontend SPA | Static file check | API proxy health | Not needed |
| Stream processor | Thread health | Offset lag check | Initial offset check |

### HTTP Status Code Semantics

| Code | Meaning | Use Case |
|------|---------|----------|
| 200 OK | Fully healthy | Standard healthy response |
| 204 No Content | Healthy, no body | Minimal response for efficiency |
| 503 Service Unavailable | Unhealthy/Not ready | Readiness probe failure |
| 500 Internal Error | Check failure | Exception during health check |
| 429 Too Many Requests | Degraded due to load | Rate limiting active |

---

*Document version: 1.0*  
*HelMo Research Initiative*  
*For questions or updates, refer to project documentation*
