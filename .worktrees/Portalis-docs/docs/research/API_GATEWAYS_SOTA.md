# API Gateways State of the Art

**Document ID:** PORTALIS-RESEARCH-001  
**Status:** Active Research  
**Last Updated:** 2026-04-02  
**Author:** Portalis Architecture Team

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Kong Gateway](#kong-gateway)
3. [Ambassador Edge Stack](#ambassador-edge-stack)
4. [Traefik Proxy](#traefik-proxy)
5. [NGINX Plus](#nginx-plus)
6. [Envoy Proxy](#envoy-proxy)
7. [AWS API Gateway](#aws-api-gateway)
8. [Azure API Management](#azure-api-management)
9. [Cloudflare API Gateway](#cloudflare-api-gateway)
10. [Comparison Matrix](#comparison-matrix)
11. [Selection Criteria](#selection-criteria)
12. [References](#references)

---

## Executive Summary

This document provides a comprehensive analysis of modern API gateway solutions for the Portalis project. API gateways serve as the critical entry point for all client traffic, handling authentication, rate limiting, request routing, protocol translation, and observability.

### Key Considerations for Portalis

- **Performance:** Sub-10ms latency overhead at p99
- **Extensibility:** Plugin/custom middleware support for custom auth
- **Kubernetes Integration:** Native ingress controller capabilities
- **Developer Experience:** Strong configuration validation and hot reloading
- **Observability:** OpenTelemetry, Prometheus, and structured logging
- **Multi-tenancy:** Support for isolated tenant configurations

---

## Kong Gateway

### Overview

Kong Gateway is a cloud-native, platform-agnostic, scalable API gateway built on NGINX and OpenResty. It uses LuaJIT for high-performance plugin execution.

```
┌─────────────────────────────────────────────────────────────────┐
│                        KONG GATEWAY ARCHITECTURE                   │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│   ┌──────────────┐    ┌──────────────┐    ┌──────────────┐       │
│   │   Admin API  │    │   Data Plane │    │  Control Plane│       │
│   │   (HTTP:8001)│    │   (Proxy:8000)│   │  (Database)   │       │
│   └──────┬───────┘    └──────┬───────┘    └──────┬───────┘       │
│          │                   │                   │               │
│          │    ┌──────────────┴───────────────┐   │               │
│          │    │                            │   │               │
│          ▼    ▼                            │   ▼               │
│   ┌─────────────────┐              ┌───────┴──────────┐          │
│   │   Plugin        │              │   Configuration  │          │
│   │   Execution     │              │   Store (DB-less)│          │
│   │   (LuaJIT)      │              │   or PostgreSQL  │          │
│   └─────────────────┘              └──────────────────┘          │
│                                                                   │
│   ┌────────────────────────────────────────────────────────┐     │
│   │                 NGINX + OpenResty Core                  │     │
│   └────────────────────────────────────────────────────────┘     │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### Architecture Components

```
┌─────────────────────────────────────────────────────────────────┐
│                     REQUEST LIFECYCLE                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│   Client Request                                                  │
│        │                                                          │
│        ▼                                                          │
│   ┌─────────────────────────────────────────────────────────┐    │
│   │  PHASE: ssl_certificate                                  │    │
│   │  - SSL handshake, SNI handling                           │    │
│   └─────────────────────────────────────────────────────────┘    │
│        │                                                          │
│        ▼                                                          │
│   ┌─────────────────────────────────────────────────────────┐    │
│   │  PHASE: rewrite                                          │    │
│   │  - URL rewriting, header manipulation                    │    │
│   │  - Kong plugins: request-transformer, key-auth             │    │
│   └─────────────────────────────────────────────────────────┘    │
│        │                                                          │
│        ▼                                                          │
│   ┌─────────────────────────────────────────────────────────┐    │
│   │  PHASE: access                                           │    │
│   │  - Authentication, rate limiting, ACL checks             │    │
│   │  - Kong plugins: jwt, oauth2, rate-limiting, cors          │    │
│   └─────────────────────────────────────────────────────────┘    │
│        │                                                          │
│        ▼                                                          │
│   ┌─────────────────────────────────────────────────────────┐    │
│   │  PHASE: balancer                                         │    │
│   │  - Upstream selection, load balancing                    │    │
│   │  - Health checks, circuit breakers                       │    │
│   └─────────────────────────────────────────────────────────┘    │
│        │                                                          │
│        ▼                                                          │
│   ┌─────────────────────────────────────────────────────────┐    │
│   │  PHASE: header_filter                                    │    │
│   │  - Response header manipulation                          │    │
│   │  - Kong plugins: response-transformer                      │    │
│   └─────────────────────────────────────────────────────────┘    │
│        │                                                          │
│        ▼                                                          │
│   ┌─────────────────────────────────────────────────────────┐    │
│   │  PHASE: body_filter                                    │    │
│   │  - Response body transformation                          │    │
│   │  - Kong plugins: grpc-gateway, file-log                    │    │
│   └─────────────────────────────────────────────────────────┘    │
│        │                                                          │
│        ▼                                                          │
│   ┌─────────────────────────────────────────────────────────┐    │
│   │  PHASE: log                                              │    │
│   │  - Access logging, metrics emission                      │    │
│   │  - Kong plugins: prometheus, datadog, http-log             │    │
│   └─────────────────────────────────────────────────────────┘    │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### Configuration Examples

#### Declarative Configuration (DB-less Mode)

```yaml
# kong.yml - Declarative Configuration
_format_version: "3.0"
_transform: true

services:
  - name: user-service
    url: http://user-api.internal:8080
    routes:
      - name: user-routes
        paths:
          - /api/v1/users
        methods:
          - GET
          - POST
          - PUT
          - DELETE
        strip_path: false
        preserve_host: true
    plugins:
      - name: rate-limiting
        config:
          minute: 100
          policy: local
      - name: jwt
        config:
          uri_param_names:
            - jwt
          cookie_names: []
          key_claim_name: iss
          secret_is_base64: false
          claims_to_verify:
            - exp
      - name: cors
        config:
          origins:
            - "https://app.portalis.io"
            - "https://admin.portalis.io"
          methods:
            - GET
            - POST
            - PUT
            - DELETE
            - OPTIONS
          headers:
            - Authorization
            - Content-Type
            - X-Request-ID
          max_age: 3600
          credentials: true

  - name: billing-service
    url: http://billing-api.internal:8081
    routes:
      - name: billing-routes
        paths:
          - /api/v1/billing
        methods:
          - GET
          - POST
    plugins:
      - name: rate-limiting
        config:
          minute: 50
          policy: redis
          redis_host: redis.internal
          redis_port: 6379
          redis_timeout: 2000
      - name: oauth2
        config:
          scopes:
            - billing:read
            - billing:write
          mandatory_scope: true
          enable_authorization_code: true
          enable_client_credentials: true
          global_credentials: false
          hide_credentials: true

consumers:
  - username: mobile-app-client
    custom_id: mobile-app-001
    jwt_secrets:
      - algorithm: RS256
        rsa_public_key: |
          -----BEGIN PUBLIC KEY-----
          MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA...
          -----END PUBLIC KEY-----
        key: "mobile-app-key"
    keyauth_credentials:
      - key: mobile-api-key-secure-123

plugins:
  - name: prometheus
    config:
      per_consumer: true
      status_code_metrics: true
      latency_metrics: true
      bandwidth_metrics: true
      upstream_health_metrics: true

  - name: file-log
    config:
      path: /var/log/kong/access.log
      reopen: true
```

#### Kubernetes Ingress Controller

```yaml
# kong-ingress.yaml
apiVersion: configuration.konghq.com/v1
kind: KongPlugin
metadata:
  name: portalis-rate-limit
  namespace: api-gateway
config:
  minute: 1000
  policy: redis
  redis_host: redis-cluster.api-gateway.svc.cluster.local
  redis_port: 6379
  fault_tolerant: true
  hide_client_headers: false
  redis_timeout: 2000
  redis_database: 0
plugin: rate-limiting
---
apiVersion: configuration.konghq.com/v1
kind: KongPlugin
metadata:
  name: portalis-jwt-auth
  namespace: api-gateway
config:
  uri_param_names:
    - jwt
  cookie_names:
    - portalis_session
  key_claim_name: iss
  secret_is_base64: false
  claims_to_verify:
    - exp
  maximum_expiration: 86400
plugin: jwt
---
apiVersion: configuration.konghq.com/v1
kind: KongClusterPlugin
metadata:
  name: global-cors
  namespace: api-gateway
  annotations:
    kubernetes.io/ingress.class: kong
config:
  origins:
    - "*"
  methods:
    - GET
    - POST
    - PUT
    - DELETE
    - PATCH
    - OPTIONS
  headers:
    - Authorization
    - Content-Type
    - X-Request-ID
    - X-Correlation-ID
    - X-API-Key
  exposed_headers:
    - X-Request-ID
    - X-RateLimit-Limit
    - X-RateLimit-Remaining
  max_age: 3600
  credentials: true
  preflight_continue: false
plugin: cors
---
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: portalis-api-ingress
  namespace: api-gateway
  annotations:
    kubernetes.io/ingress.class: kong
    konghq.com/strip-path: "true"
    konghq.com/preserve-host: "true"
    konghq.com/plugins: portalis-rate-limit,portalis-jwt-auth
spec:
  rules:
    - host: api.portalis.io
      http:
        paths:
          - path: /v1/users
            pathType: Prefix
            backend:
              service:
                name: user-service
                port:
                  number: 80
          - path: /v1/billing
            pathType: Prefix
            backend:
              service:
                name: billing-service
                port:
                  number: 80
          - path: /v1/analytics
            pathType: Prefix
            backend:
              service:
                name: analytics-service
                port:
                  number: 80
---
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: portalis-public-ingress
  namespace: api-gateway
  annotations:
    kubernetes.io/ingress.class: kong
    konghq.com/strip-path: "true"
    konghq.com/plugins: portalis-rate-limit
spec:
  rules:
    - host: api.portalis.io
      http:
        paths:
          - path: /v1/public
            pathType: Prefix
            backend:
              service:
                name: public-service
                port:
                  number: 80
```

### Key Features

| Feature | Description | Configuration |
|---------|-------------|---------------|
| **Plugin System** | 100+ built-in plugins | LuaJIT execution |
| **Authentication** | JWT, OAuth2, OIDC, LDAP, mTLS | Per-route or global |
| **Rate Limiting** | Token bucket, sliding window | Redis/DB-backed |
| **Traffic Control** | Circuit breaker, retries, timeouts | Upstream config |
| **Observability** | Prometheus, StatsD, Datadog | Plugin-based |
| **Caching** | Proxy caching with TTL | Redis-backed |
| **Transformations** | Request/response body/headers | JSON/XML support |
| **Load Balancing** | Round-robin, least-connections, consistent hashing | Health checks |
| **Service Mesh** | Kong Mesh (Istio alternative) | mTLS, traffic split |
| **Developer Portal** | Customizable API documentation | OpenAPI integration |

### Performance Characteristics

```
┌─────────────────────────────────────────────────────────────────┐
│                    KONG PERFORMANCE PROFILE                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│   Latency (p99): 1-5ms (no plugins)                               │
│                  5-15ms (with auth + rate limiting)              │
│                  15-30ms (with request transformation)           │
│                                                                   │
│   Throughput:    50,000+ RPS per node                            │
│                                                                   │
│   Memory:        ~500MB base + plugins                           │
│                                                                   │
│   Scalability:   Horizontal via load balancer                    │
│                  Stateless data plane                            │
│                                                                   │
│   Connection Pooling: Keep-alive by default                      │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### Pros and Cons

**Pros:**
- Mature ecosystem with extensive plugin library
- Excellent performance (LuaJIT)
- Strong Kubernetes integration (Ingress Controller)
- Good documentation and community
- Enterprise support available (Kong Inc)
- Hybrid mode (DB-less for data plane)

**Cons:**
- Lua plugin development has learning curve
- Kong Manager (UI) is enterprise-only
- Complex configuration for advanced scenarios
- PostgreSQL dependency for control plane

---

## Ambassador Edge Stack

### Overview

Ambassador Edge Stack is a Kubernetes-native API gateway built on Envoy Proxy. It uses Custom Resource Definitions (CRDs) for configuration and provides a declarative approach to API management.

```
┌─────────────────────────────────────────────────────────────────┐
│                 AMBASSADOR EDGE STACK ARCHITECTURE               │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │              Ambassador Control Plane                      │   │
│   │  - Watches Kubernetes API for CRD changes                │   │
│   │  - Generates Envoy configuration                        │   │
│   │  - Hot-reloads Envoy without restarts                   │   │
│   └──────────────────────┬──────────────────────────────────┘   │
│                          │                                       │
│                          ▼ xDS API                                │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                   Envoy Proxy Data Plane                 │   │
│   │                                                          │   │
│   │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌────────┐  │   │
│   │  │ Listener │  │   Route  │  │  Cluster │  │Endpoint│  │   │
│   │  │  :8080   │──▶│   Table  │──▶│Discovery │──▶│  EDS   │  │   │
│   │  └──────────┘  └──────────┘  └──────────┘  └────────┘  │   │
│   │                                                          │   │
│   │  ┌───────────────────────────────────────────────────┐   │   │
│   │  │            HTTP Connection Manager               │   │   │
│   │  │   - Rate limiting, Auth, Routing, CORS          │   │   │
│   │  └───────────────────────────────────────────────────┘   │   │
│   │                                                          │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                   │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │              Ambassador Custom Resources                 │   │
│   │                                                          │   │
│   │  • Host         - Domain and TLS configuration          │   │
│   │  • Mapping      - Route definitions                     │   │
│   │  • Module       - Global configuration                  │   │
│   │  • RateLimit    - Rate limiting policies                │   │
│   │  • AuthService  - External authentication               │   │
│   │  • LogService   - Access logging configuration          │   │
│   │  • Listener     - Port and protocol settings            │   │
│   │  • TCPMapping   - TCP route definitions                 │   │
│   │                                                          │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### Architecture Flow

```
┌─────────────────────────────────────────────────────────────────┐
│              AMBASSADOR CONFIGURATION FLOW                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│   Kubernetes API                                                  │
│        │                                                          │
│        │ Watch CRDs                                               │
│        ▼                                                          │
│   ┌─────────────────┐                                             │
│   │  Ambassador     │                                             │
│   │  Control Plane  │                                             │
│   │                 │                                             │
│   │  • Parse CRDs   │                                             │
│   │  • Validate     │                                             │
│   │  • Generate     │                                             │
│   └────────┬────────┘                                             │
│            │                                                      │
│            │ Generate Envoy Config                                  │
│            ▼                                                      │
│   ┌─────────────────┐                                             │
│   │  Envoy Snapshot │                                             │
│   │  (XDS Cache)    │                                             │
│   └────────┬────────┘                                             │
│            │                                                      │
│            │ gRPC XDS                                             │
│            ▼                                                      │
│   ┌─────────────────┐                                             │
│   │  Envoy Proxy    │                                             │
│   │  Hot Reload     │                                             │
│   └─────────────────┘                                             │
│                                                                   │
│   Reload Time: < 100ms (no dropped connections)                  │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### Configuration Examples

#### Basic Mapping Configuration

```yaml
# ambassador-mappings.yaml
apiVersion: getambassador.io/v3alpha1
kind: Host
metadata:
  name: portalis-host
  namespace: api-gateway
spec:
  hostname: api.portalis.io
  acmeProvider:
    authority: https://acme-v02.api.letsencrypt.org/directory
    email: admin@portalis.io
  tlsSecret:
    name: portalis-tls-cert
  requestPolicy:
    insecure:
      action: Redirect
      additionalPort: 8080
---
apiVersion: getambassador.io/v3alpha1
kind: Mapping
metadata:
  name: user-service-mapping
  namespace: api-gateway
spec:
  hostname: api.portalis.io
  prefix: /api/v1/users
  service: user-service.api-gateway:8080
  timeout_ms: 30000
  connect_timeout_ms: 10000
  idle_timeout_ms: 300000
  retry_policy:
    retry_on: 5xx
    num_retries: 3
    per_try_timeout: 10000
  circuit_breakers:
    max_connections: 1000
    max_pending_requests: 1000
    max_requests: 1000
    max_retries: 3
  cors:
    origins:
      - "https://app.portalis.io"
      - "https://admin.portalis.io"
    methods: GET, POST, PUT, DELETE, OPTIONS, PATCH
    headers:
      - Authorization
      - Content-Type
      - X-Request-ID
      - X-API-Key
    credentials: true
    max_age: "3600"
  docs:
    path: /api/v1/users/openapi.json
    display_name: "User Service API"
---
apiVersion: getambassador.io/v3alpha1
kind: Mapping
metadata:
  name: billing-service-mapping
  namespace: api-gateway
spec:
  hostname: api.portalis.io
  prefix: /api/v1/billing
  service: billing-service.api-gateway:8080
  rate_limits:
    - action: Request
      scope: billing-api
      descriptors:
        - generic_key:
            value: billing-endpoint
  timeout_ms: 45000
  rewrite: /api/v1/billing
  add_request_headers:
    x-forwarded-portal:
      value: "true"
    x-portal-version:
      value: "v1"
  remove_request_headers:
    - x-internal-token
---
apiVersion: getambassador.io/v3alpha1
kind: RateLimit
metadata:
  name: portalis-rate-limits
  namespace: api-gateway
spec:
  domain: ambassador
  service: "rate-limit-service.api-gateway:8081"
  protocol_version: v3
  timeout_ms: 20
  failure_mode_deny: false
  rate_limits:
    - name: general-api-limit
      action: Request
      scope: global
      descriptors:
        - generic_key:
            value: api-request
      limits:
        - action: Request
          unit: minute
          requests: 1000
    - name: billing-specific-limit
      action: Request
      scope: billing-api
      descriptors:
        - generic_key:
            value: billing-endpoint
      limits:
        - action: Request
          unit: minute
          requests: 100
    - name: authenticated-user-limit
      action: Request
      scope: user-scope
      descriptors:
        - remote_address: {}
      limits:
        - action: Request
          unit: minute
          requests: 500
```

#### Authentication Configuration

```yaml
# ambassador-auth.yaml
apiVersion: getambassador.io/v3alpha1
kind: AuthService
metadata:
  name: portalis-authentication
  namespace: api-gateway
spec:
  auth_service: "auth-service.api-gateway:8080"
  proto: http
  timeout_ms: 5000
  failure_mode_allow: false
  failure_mode_allow_header: true
  include_body:
    max_bytes: 4096
    allow_partial: true
  status_on_error:
    code: 503
  allowed_request_headers:
    - x-request-id
    - x-forwarded-for
    - x-forwarded-proto
    - x-forwarded-host
    - x-real-ip
    - authorization
    - cookie
  allowed_authorization_headers:
    - authorization
    - set-cookie
    - x-user-id
    - x-user-role
    - x-tenant-id
    - x-request-id
  add_auth_headers:
    x-authenticated:
      value: "true"
  add_linkerd_headers: true
---
apiVersion: getambassador.io/v3alpha1
kind: Mapping
metadata:
  name: public-api-mapping
  namespace: api-gateway
spec:
  hostname: api.portalis.io
  prefix: /api/v1/public/
  service: public-service.api-gateway:8080
  bypass_auth: true
  rate_limits:
    - action: Request
      scope: public-api
      descriptors:
        - generic_key:
            value: public-endpoint
```

#### Advanced Load Balancing

```yaml
# ambassador-loadbalancing.yaml
apiVersion: getambassador.io/v3alpha1
kind: Mapping
metadata:
  name: analytics-service-mapping
  namespace: api-gateway
spec:
  hostname: api.portalis.io
  prefix: /api/v1/analytics
  service: analytics-service.api-gateway:8080
  resolver: endpoint
  load_balancer:
    policy: ring_hash
    ring_hash_lb_config:
      minimum_ring_size: 1024
      maximum_ring_size: 8388608
  headers:
    x-tenant-id:
      - regex: "tenant-.*"
  # Canary deployment - 10% traffic to v2
  weight: 90
---
apiVersion: getambassador.io/v3alpha1
kind: Mapping
metadata:
  name: analytics-service-v2-mapping
  namespace: api-gateway
spec:
  hostname: api.portalis.io
  prefix: /api/v1/analytics
  service: analytics-service-v2.api-gateway:8080
  resolver: endpoint
  load_balancer:
    policy: ring_hash
  weight: 10
  headers:
    x-canary:
      value: "true"
      regex: false
```

### Key Features

| Feature | Description | Implementation |
|---------|-------------|----------------|
| **GitOps Native** | CRD-based configuration | Kubernetes native |
| **Hot Reload** | Zero-downtime config updates | Envoy XDS API |
| **Edge Stack** | Integrated developer portal | Built-in |
| **Rate Limiting** | Envoy rate limit service | External service |
| **Authentication** | External auth service | Webhook pattern |
| **Observability** | Prometheus, Grafana, Jaeger | Built-in |
| **Load Balancing** | Advanced algorithms | Envoy native |
| **Circuit Breakers** | Automatic failure detection | Envoy native |
| **Retries** | Configurable retry policies | Envoy native |
| **Timeouts** | Per-route timeout config | Envoy native |

### Performance Characteristics

```
┌─────────────────────────────────────────────────────────────────┐
│                AMBASSADOR PERFORMANCE PROFILE                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│   Latency (p99): 1-3ms (Envoy native)                            │
│                  5-10ms (with external auth)                     │
│                  10-20ms (with rate limiting)                    │
│                                                                   │
│   Throughput:    100,000+ RPS per node (Envoy)                   │
│                                                                   │
│   Memory:        ~200MB base + configuration                     │
│                                                                   │
│   Scalability:   Horizontal via Kubernetes                       │
│                  Control plane scales independently              │
│                                                                   │
│   Configuration Lag: < 100ms (CRD to Envoy)                      │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### Pros and Cons

**Pros:**
- Native Kubernetes integration
- Envoy Proxy foundation (high performance)
- Excellent GitOps workflow
- Strong developer portal (Edge Stack)
- Good observability out of the box
- No database dependency

**Cons:**
- Kubernetes-only deployment
- Smaller plugin ecosystem than Kong
- External rate limiting service required
- Learning curve for Envoy concepts
- Enterprise features in paid tiers

---

## Traefik Proxy

### Overview

Traefik is a cloud-native edge router that automatically discovers services and configures itself. It excels in dynamic environments with its provider-based configuration system.

```
┌─────────────────────────────────────────────────────────────────┐
│                     TRAEFIK ARCHITECTURE                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                  Providers (Configuration Sources)         │   │
│   │                                                          │   │
│   │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌────────┐  │   │
│   │  │Kubernetes│  │  Docker  │  │   Consul │  │  File  │  │   │
│   │  │  CRD     │  │  Labels  │  │  Catalog │  │  TOML  │  │   │
│   │  └──────────┘  └──────────┘  └──────────┘  └────────┘  │   │
│   │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌────────┐  │   │
│   │  │   ECS    │  │  Rancher │  │   Nomad  │  │  Etcd  │  │   │
│   │  └──────────┘  └──────────┘  └──────────┘  └────────┘  │   │
│   │                                                          │   │
│   └─────────────────────────┬───────────────────────────────┘   │
│                             │                                     │
│                             ▼                                     │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                  Traefik Configuration                  │   │
│   │                                                          │   │
│   │  • EntryPoints (Ports/Protocols)                        │   │
│   │  • Routers (Request matching rules)                     │   │
│   │  • Services (Backend definitions)                       │   │
│   │  • Middlewares (Request processing)                   │   │
│   │  • Certificates (TLS)                                 │   │
│   │                                                          │   │
│   └─────────────────────────┬───────────────────────────────┘   │
│                             │                                     │
│                             ▼                                     │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                    Traefik Core                         │   │
│   │                                                          │   │
│   │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌────────┐  │   │
│   │  │  HTTP    │  │   TCP    │  │   UDP    │  │  TLS   │  │   │
│   │  │  Router  │  │  Router  │  │  Router  │  │ Handler│  │   │
│   │  └──────────┘  └──────────┘  └──────────┘  └────────┘  │   │
│   │                                                          │   │
│   │  ┌───────────────────────────────────────────────────┐ │   │
│   │  │              Middleware Chain                        │ │   │
│   │  │  Auth ▶ RateLimit ▶ Redirect ▶ Headers ▶ Forward  │ │   │
│   │  └───────────────────────────────────────────────────┘ │   │
│   │                                                          │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### Configuration Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                TRAEFIK CONFIGURATION HIERARCHY                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│   Static Configuration (traefik.yml / CLI flags)                  │
│   ├─ EntryPoints (ports, protocols)                               │
│   ├─ Providers (discovery sources)                                │
│   ├─ API/Dashboard settings                                       │
│   ├─ Certificate resolvers                                        │
│   └─ Logging/Tracing                                              │
│                                                                   │
│                         │                                         │
│                         ▼                                         │
│                                                                   │
│   Dynamic Configuration (providers)                               │
│   ├─ Routers                                                      │
│   │  ├─ Rule: "Host(`api.portalis.io`) && PathPrefix(`/v1/`)"   │
│   │  ├─ EntryPoints: ["websecure"]                               │
│   │  ├─ Service: "user-service"                                    │
│   │  ├─ Middlewares: ["auth@file", "ratelimit@file"]              │
│   │  └─ TLS: certResolver: "letsencrypt"                          │
│   │                                                              │
│   ├─ Services                                                     │
│   │  ├─ Type: loadBalancer                                       │
│   │  ├─ Servers: [{url: "http://svc:8080"}]                    │
│   │  ├─ HealthCheck: {path: "/health", interval: "10s"}          │
│   │  └─ LoadBalancer: {method: "wrr", sticky: {cookie: {}}}      │
│   │                                                              │
│   └─ Middlewares                                                  │
│      ├─ basicAuth, digestAuth, forwardAuth                         │
│      ├─ rateLimit, inFlightReq, buffering                         │
│      ├─ circuitBreaker, retry, errorPage                           │
│      ├─ addPrefix, stripPrefix, replacePath                        │
│      ├─ headers, customRequestHeaders, customResponseHeaders       │
│      └─ chain (middleware composition)                             │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### Configuration Examples

#### Static Configuration

```yaml
# traefik.yml - Static Configuration
global:
  checkNewVersion: false
  sendAnonymousUsage: false

api:
  dashboard: true
  insecure: false
  basePath: /dashboard

trying:
  dashboard: true
  basePath: /api

entryPoints:
  web:
    address: ":80"
    http:
      redirections:
        entryPoint:
          to: websecure
          scheme: https
          permanent: true

  websecure:
    address: ":443"
    http:
      tls:
        certResolver: letsencrypt
        options: default
      middlewares:
        - security-headers@file

  traefik:
    address: ":8080"

providers:
  kubernetesCRD:
    namespaces:
      - api-gateway
      - default
    allowCrossNamespace: false
    allowExternalNameServices: false

  file:
    directory: /etc/traefik/dynamic
    watch: true

certificatesResolvers:
  letsencrypt:
    acme:
      email: admin@portalis.io
      storage: /letsencrypt/acme.json
      tlsChallenge: {}
      httpChallenge:
        entryPoint: web

  internal:
    acme:
      email: admin@portalis.io
      storage: /letsencrypt/internal.json
      caServer: https://ca.internal/acme/directory
      tlsChallenge: {}

log:
  level: INFO
  format: json
  filePath: /var/log/traefik/traefik.log

accessLog:
  format: json
  filePath: /var/log/traefik/access.log
  bufferingSize: 100
  filters:
    statusCodes:
      - "400-499"
      - "500-599"
    retryAttempts: true
    minDuration: "10ms"
  fields:
    names:
      ClientUsername: drop
    headers:
      defaultMode: drop
      names:
        User-Agent: keep
        X-Request-ID: keep
        X-Correlation-ID: keep

tracing:
  serviceName: portalis-gateway
  jaeger:
    samplingServerURL: http://jaeger-agent:5778/sampling
    localAgentHostPort: jaeger-agent:6831

metrics:
  prometheus:
    addEntryPointsLabels: true
    addServicesLabels: true
    addRoutersLabels: true
```

#### Dynamic Configuration (File Provider)

```yaml
# /etc/traefik/dynamic/middlewares.yml
http:
  middlewares:
    security-headers:
      headers:
        customRequestHeaders:
          X-Forwarded-Proto: "https"
        customResponseHeaders:
          X-Frame-Options: "DENY"
          X-Content-Type-Options: "nosniff"
          X-XSS-Protection: "1; mode=block"
          Strict-Transport-Security: "max-age=31536000; includeSubDomains"
          Referrer-Policy: "strict-origin-when-cross-origin"
        accessControlAllowMethods:
          - GET
          - POST
          - PUT
          - DELETE
          - OPTIONS
        accessControlAllowHeaders:
          - Authorization
          - Content-Type
          - X-Request-ID
        accessControlAllowOriginList:
          - "https://app.portalis.io"
          - "https://admin.portalis.io"
        accessControlMaxAge: 3600
        addVaryHeader: true

    rate-limit-general:
      rateLimit:
        average: 100
        burst: 50
        period: 1m
        sourceCriterion:
          requestHeaderName:
            headerName: X-API-Key

    rate-limit-strict:
      rateLimit:
        average: 10
        burst: 5
        period: 1m
        sourceCriterion:
          ipStrategy:
            depth: 1
            excludedIPs:
              - 10.0.0.0/8
              - 172.16.0.0/12

    circuit-breaker:
      circuitBreaker:
        expression: |
          LatencyAtQuantileMS(50.0) > 100
          || ResponseCodeRatio(500, 600, 0, 600) > 0.25
          || NetworkErrorRatio() > 0.5
        checkPeriod: 10s
        fallbackDuration: 10s
        recoveryDuration: 10s

    retry-policy:
      retry:
        attempts: 3
        initialInterval: 100ms

    compress-response:
      compress: {}

    auth-forward:
      forwardAuth:
        address: http://auth-service.api-gateway:8080/verify
        trustForwardHeader: true
        authResponseHeaders:
          - X-User-ID
          - X-User-Role
          - X-Tenant-ID
          - X-Authenticated
        authRequestHeaders:
          - Authorization
          - Cookie
          - X-Request-ID

    strip-api-prefix:
      stripPrefix:
        prefixes:
          - /api/v1
        forceSlash: true

    add-request-id:
      plugin:
        requestid:
          headerName: X-Request-ID

  routers:
    dashboard:
      rule: Host(`traefik.portalis.io`)
      service: api@internal
      middlewares:
        - auth-forward
      tls:
        certResolver: letsencrypt

tcp:
  middlewares:
    proxy-protocol:
      ipAllowList:
        sourceRange:
          - 10.0.0.0/8
          - 172.16.0.0/12
```

#### Kubernetes CRD Configuration

```yaml
# traefik-crd.yaml
apiVersion: traefik.io/v1alpha1
kind: Middleware
metadata:
  name: portalis-auth
  namespace: api-gateway
spec:
  forwardAuth:
    address: http://auth-service.api-gateway.svc.cluster.local:8080/verify
    trustForwardHeader: true
    authResponseHeaders:
      - X-User-ID
      - X-User-Role
      - X-Tenant-ID
      - Authorization
    authRequestHeaders:
      - Authorization
      - Cookie
      - X-Request-ID
      - X-Forwarded-For
---
apiVersion: traefik.io/v1alpha1
kind: Middleware
metadata:
  name: portalis-rate-limit
  namespace: api-gateway
spec:
  rateLimit:
    average: 100
    burst: 50
    period: 1m0s
    sourceCriterion:
      requestHeaderName:
        headerName: X-API-Key
---
apiVersion: traefik.io/v1alpha1
kind: Middleware
metadata:
  name: portalis-circuit-breaker
  namespace: api-gateway
spec:
  circuitBreaker:
    expression: |
      LatencyAtQuantileMS(50.0) > 100
      || ResponseCodeRatio(500, 600, 0, 600) > 0.25
    checkPeriod: 10s
---
apiVersion: traefik.io/v1alpha1
kind: Middleware
metadata:
  name: portalis-retry
  namespace: api-gateway
spec:
  retry:
    attempts: 3
    initialInterval: 100ms
---
apiVersion: traefik.io/v1alpha1
kind: IngressRoute
metadata:
  name: portalis-api-route
  namespace: api-gateway
spec:
  entryPoints:
    - websecure
  routes:
    - match: Host(`api.portalis.io`) && PathPrefix(`/v1/users`)
      kind: Rule
      services:
        - name: user-service
          port: 8080
          healthCheck:
            path: /health
            interval: 10s
            timeout: 5s
          strategy: RoundRobin
          weight: 100
      middlewares:
        - name: portalis-auth
        - name: portalis-rate-limit
        - name: portalis-circuit-breaker

    - match: Host(`api.portalis.io`) && PathPrefix(`/v1/billing`)
      kind: Rule
      services:
        - name: billing-service
          port: 8080
          healthCheck:
            path: /health
            interval: 10s
      middlewares:
        - name: portalis-auth
        - name: portalis-rate-limit
        - name: portalis-retry

    - match: Host(`api.portalis.io`) && PathPrefix(`/v1/public`)
      kind: Rule
      services:
        - name: public-service
          port: 8080
      middlewares:
        - name: portalis-rate-limit

  tls:
    certResolver: letsencrypt
    options: default
---
apiVersion: traefik.io/v1alpha1
kind: ServersTransport
metadata:
  name: portalis-backend-tls
  namespace: api-gateway
spec:
  serverName: ""
  insecureSkipVerify: false
  rootCAsSecrets:
    - backend-ca-cert
  certificatesSecrets:
    - backend-client-cert
  maxIdleConnsPerHost: 100
  forwardingTimeouts:
    dialTimeout: 30s
    responseHeaderTimeout: 10s
    idleConnTimeout: 90s
```

### Key Features

| Feature | Description | Traefik Equivalent |
|---------|-------------|-------------------|
| **Auto Discovery** | Automatic service detection | Provider system |
| **Middleware Chain** | Composable request processing | Middlewares |
| **Let's Encrypt** | Automatic TLS | Built-in ACME |
| **Dashboard** | Web UI for monitoring | Built-in |
| **Circuit Breaker** | Automatic failure detection | Middleware |
| **Rate Limiting** | Token bucket algorithm | Middleware |
| **Load Balancing** | WRR, Round Robin, IP Hash | Service config |
| **Retry Logic** | Configurable retries | Middleware |
| **Tracing** | OpenTracing/OpenTelemetry | Built-in |
| **Metrics** | Prometheus export | Built-in |

### Performance Characteristics

```
┌─────────────────────────────────────────────────────────────────┐
│                 TRAEFIK PERFORMANCE PROFILE                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│   Latency (p99): 0.5-2ms (no middlewares)                        │
│                  2-8ms (with auth + rate limit)                  │
│                  5-15ms (with circuit breaker)                   │
│                                                                   │
│   Throughput:    50,000+ RPS per node                              │
│                                                                   │
│   Memory:        ~100MB base + dynamic config                    │
│                                                                   │
│   Scalability:   Horizontal via Kubernetes/Service Mesh            │
│                                                                   │
│   Configuration Lag: < 1s (provider polling)                     │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### Pros and Cons

**Pros:**
- Simple, intuitive configuration
- Excellent documentation
- Great for Docker/Kubernetes environments
- Built-in Let's Encrypt support
- Beautiful dashboard UI
- Active community
- Fast configuration hot-reloading

**Cons:**
- Less mature enterprise features
- Smaller plugin ecosystem
- Lua plugin support limited (Traefik v3)
- Enterprise features in Traefik Enterprise
- Some advanced features require plugins

---

## NGINX Plus

### Overview

NGINX Plus is the commercial version of NGINX, providing advanced load balancing, monitoring, and management features. It's battle-tested and widely deployed in production.

```
┌─────────────────────────────────────────────────────────────────┐
│                    NGINX PLUS ARCHITECTURE                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                    NGINX Plus Core                       │   │
│   │                                                          │   │
│   │  ┌────────────┐  ┌────────────┐  ┌────────────┐         │   │
│   │  │   HTTP     │  │   Stream   │  │   Mail     │         │   │
│   │  │   Module   │  │   Module   │  │   Module   │         │   │
│   │  └─────┬──────┘  └─────┬──────┘  └─────┬──────┘         │   │
│   │        │               │               │                │   │
│   │        └───────────────┼───────────────┘                │   │
│   │                        │                                │   │
│   │                        ▼                                │   │
│   │  ┌───────────────────────────────────────────────────┐  │   │
│   │  │              Event-Driven Core (kqueue/epoll)     │  │   │
│   │  └───────────────────────────────────────────────────┘  │   │
│   │                                                          │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                   │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │              NGINX Plus Exclusive Features               │   │
│   │                                                          │   │
│   │  ┌────────────┐  ┌────────────┐  ┌────────────┐         │   │
│   │  │    API     │  │ Dashboard  │  │   Health   │         │   │
│   │  │  (REST)    │  │    (UI)    │  │   Checks   │         │   │
│   │  └────────────┘  └────────────┘  └────────────┘         │   │
│   │  ┌────────────┐  ┌────────────┐  ┌────────────┐         │   │
│   │  │  Session   │  │   Active   │  │    Key-    │         │   │
│   │  │  Draining  │  │   Health   │  │ Value Store│         │   │
│   │  └────────────┘  └────────────┘  └────────────┘         │   │
│   │                                                          │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                   │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │               Third-Party Modules                        │   │
│   │                                                          │   │
│   │  • njs (NGINX JavaScript) - nginScript                  │   │
│   │  • njs modules for custom logic                          │   │
│   │  • OpenTracing module                                    │   │
│   │  • Prometheus exporter                                   │   │
│   │  • LDAP authentication module                            │   │
│   │                                                          │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### Configuration Examples

```nginx
# nginx.conf - NGINX Plus API Gateway Configuration

user nginx;
worker_processes auto;
worker_rlimit_nofile 65535;
error_log /var/log/nginx/error.log warn;
pid /var/run/nginx.pid;

events {
    worker_connections 16384;
    use epoll;
    multi_accept on;
}

http {
    include /etc/nginx/mime.types;
    default_type application/json;

    # Logging Format
    log_format main '$remote_addr - $remote_user [$time_local] "$request" '
                    '$status $body_bytes_sent "$http_referer" '
                    '"$http_user_agent" "$http_x_forwarded_for" '
                    'rt=$request_time uct="$upstream_connect_time" '
                    'uht="$upstream_header_time" urt="$upstream_response_time" '
                    'rid="$request_id" tenant="$http_x_tenant_id"';

    access_log /var/log/nginx/access.log main;

    # Performance Tuning
    sendfile on;
    tcp_nopush on;
    tcp_nodelay on;
    keepalive_timeout 65;
    keepalive_requests 1000;
    types_hash_max_size 2048;
    client_max_body_size 50m;
    client_body_buffer_size 128k;
    client_header_buffer_size 4k;
    large_client_header_buffers 4 8k;

    # Rate Limiting Zones
    limit_req_zone $binary_remote_addr zone=general:10m rate=100r/m;
    limit_req_zone $http_x_api_key zone=apikey:10m rate=1000r/m;
    limit_req_zone $http_x_tenant_id zone=tenant:10m rate=500r/m;
    limit_conn_zone $binary_remote_addr zone=addr:10m;

    # Upstream Definitions with Health Checks
    upstream user_service {
        zone user_service 64k;
        least_conn;
        server user-api-1.internal:8080 weight=5;
        server user-api-2.internal:8080 weight=5;
        server user-api-3.internal:8080 weight=5 backup;

        keepalive 32;
        keepalive_requests 1000;
        keepalive_timeout 60s;

        # NGINX Plus Health Checks
        health_check interval=5s fails=3 passes=2
                       uri=/health
                       match=health_check;
    }

    upstream billing_service {
        zone billing_service 64k;
        server billing-api-1.internal:8080;
        server billing-api-2.internal:8080;

        sticky cookie srv_id expires=1h domain=.portalis.io path=/;

        health_check interval=10s fails=3 passes=2
                       uri=/health
                       match=health_check;
    }

    # Health Check Match Definition
    match health_check {
        status 200;
        header Content-Type = application/json;
        body ~ "healthy";
    }

    # Key-Value Store for Dynamic Configuration
    keyval_zone zone=api_keys:1m state=/var/lib/nginx/api_keys.json;
    keyval $http_x_api_key $api_tenant zone=api_keys;

    keyval_zone zone=rate_limits:10m state=/var/lib/nginx/rate_limits.json;
    keyval $http_x_tenant_id $tenant_rate_limit zone=rate_limits;

    # Server Block - HTTP Redirect
    server {
        listen 80;
        server_name api.portalis.io;
        return 301 https://$server_name$request_uri;
    }

    # Server Block - HTTPS API Gateway
    server {
        listen 443 ssl http2;
        server_name api.portalis.io;

        # SSL Configuration
        ssl_certificate /etc/nginx/ssl/portalis.crt;
        ssl_certificate_key /etc/nginx/ssl/portalis.key;
        ssl_protocols TLSv1.2 TLSv1.3;
        ssl_ciphers ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256;
        ssl_prefer_server_ciphers off;
        ssl_session_cache shared:SSL:50m;
        ssl_session_timeout 1d;
        ssl_session_tickets off;

        # Security Headers
        add_header X-Frame-Options "SAMEORIGIN" always;
        add_header X-Content-Type-Options "nosniff" always;
        add_header X-XSS-Protection "1; mode=block" always;
        add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
        add_header Referrer-Policy "strict-origin-when-cross-origin" always;

        # Generate Request ID
        add_header X-Request-ID $request_id always;

        # CORS Preflight
        location = /OPTIONS {
            add_header Access-Control-Allow-Origin "https://app.portalis.io" always;
            add_header Access-Control-Allow-Methods "GET, POST, PUT, DELETE, OPTIONS" always;
            add_header Access-Control-Allow-Headers "Authorization, Content-Type, X-Request-ID" always;
            add_header Access-Control-Allow-Credentials "true" always;
            add_header Access-Control-Max-Age 3600 always;
            return 204;
        }

        # User Service Routes
        location /api/v1/users {
            # Rate Limiting
            limit_req zone=apikey burst=20 nodelay;
            limit_conn addr 10;

            # Authentication via subrequest
            auth_request /auth;
            auth_request_set $auth_user $upstream_http_x_user_id;
            auth_request_set $auth_role $upstream_http_x_user_role;

            proxy_pass http://user_service;
            proxy_http_version 1.1;
            proxy_set_header Connection "";

            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
            proxy_set_header X-Request-ID $request_id;
            proxy_set_header X-User-ID $auth_user;
            proxy_set_header X-User-Role $auth_role;

            proxy_connect_timeout 5s;
            proxy_send_timeout 30s;
            proxy_read_timeout 30s;

            # Error handling
            error_page 500 502 503 504 /error.html;
        }

        # Billing Service Routes
        location /api/v1/billing {
            limit_req zone=tenant burst=50 nodelay;

            auth_request /auth;

            proxy_pass http://billing_service;
            proxy_http_version 1.1;
            proxy_set_header Connection "";

            proxy_set_header Host $host;
            proxy_set_header X-Request-ID $request_id;
            proxy_set_header X-Tenant-ID $http_x_tenant_id;

            proxy_cache_bypass $http_cache_control;
        }

        # Authentication Subrequest
        location = /auth {
            internal;
            proxy_pass http://auth-service.internal:8080/verify;
            proxy_pass_request_body off;
            proxy_set_header Content-Length "";
            proxy_set_header X-Original-URI $request_uri;
            proxy_set_header X-Original-Method $request_method;
            proxy_set_header Authorization $http_authorization;
        }

        # API Status Endpoint
        location /nginx_status {
            # NGINX Plus Dashboard
            api write=on;
            allow 10.0.0.0/8;
            deny all;
        }

        # Dashboard UI
        location /dashboard {
            # NGINX Plus Dashboard UI
            dashboard;
            allow 10.0.0.0/8;
            deny all;
        }
    }
}

# njs script for custom logic
js_import /etc/nginx/njs/portalis.js;
js_set $validated_token portalis.validateToken;
```

```javascript
// /etc/nginx/njs/portalis.js
// NGINX JavaScript Module for custom authentication logic

function validateToken(r) {
    var token = r.headersIn['Authorization'];
    if (!token) {
        r.error('Missing Authorization header');
        return '';
    }

    // Remove Bearer prefix
    if (token.startsWith('Bearer ')) {
        token = token.substring(7);
    }

    // Basic JWT structure validation
    var parts = token.split('.');
    if (parts.length !== 3) {
        r.error('Invalid JWT format');
        return '';
    }

    try {
        var payload = JSON.parse(Buffer.from(parts[1], 'base64url').toString());

        // Check expiration
        if (payload.exp && payload.exp < Math.floor(Date.now() / 1000)) {
            r.error('Token expired');
            return '';
        }

        return payload.sub || '';
    } catch (e) {
        r.error('Failed to parse JWT: ' + e.message);
        return '';
    }
}

function checkTenantAccess(r) {
    var tenantId = r.headersIn['X-Tenant-ID'];
    var userId = r.variables.validated_token;

    if (!tenantId || !userId) {
        r.return(403, JSON.stringify({
            error: 'Access denied',
            message: 'Missing tenant or user identification'
        }));
        return;
    }

    // Tenant access check would go here
    // This is a placeholder for the actual implementation
    r.internalRedirect('@backend');
}

export default { validateToken, checkTenantAccess };
```

### Key Features

| Feature | Open Source | NGINX Plus |
|---------|-------------|------------|
| **Load Balancing** | Basic methods | Advanced methods + sticky sessions |
| **Health Checks** | Passive only | Active + Passive |
| **Dashboard** | Stub status | Full dashboard + API |
| **Session Draining** | No | Yes |
| **Key-Value Store** | No | Yes (dynamic config) |
| **DNS Resolution** | Basic | Advanced with SRV |
| **JWT Validation** | Limited | Full support |
| **Rate Limiting** | Basic | Advanced with variables |
| **Support** | Community | Commercial |

### Performance Characteristics

```
┌─────────────────────────────────────────────────────────────────┐
│                 NGINX PLUS PERFORMANCE PROFILE                   │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│   Latency (p99): 0.1-1ms (proxy only)                            │
│                  1-5ms (with auth subrequest)                      │
│                  5-10ms (with njs processing)                    │
│                                                                   │
│   Throughput:    200,000+ RPS per node                             │
│                                                                   │
│   Memory:        ~50MB base + connections                          │
│                                                                   │
│   Scalability:   Horizontal via DNS/Load Balancer                  │
│                  Master-worker process model                      │
│                                                                   │
│   Connection Handling: Event-driven (epoll/kqueue)               │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### Pros and Cons

**Pros:**
- Battle-tested and extremely stable
- Best-in-class performance
- Massive ecosystem and community
- Flexible configuration language
- NJS for custom logic
- Excellent documentation

**Cons:**
- Configuration complexity
- No native hot-reloading (requires reload signal)
- Plus features require commercial license
- API management features require additional modules
- No built-in service discovery (external required)

---

## Envoy Proxy

### Overview

Envoy is a high-performance C++ distributed proxy designed for single services and applications, as well as a communications bus and "universal data plane" for large mesh architectures.

```
┌─────────────────────────────────────────────────────────────────┐
│                      ENVOY ARCHITECTURE                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                   Envoy Process Structure                │   │
│   │                                                          │   │
│   │  ┌───────────────────────────────────────────────────┐   │   │
│   │  │                  Main Thread                      │   │   │
│   │  │  • XDS configuration updates                    │   │   │
│   │  │  • Stats flushing                               │   │   │
│   │  │  • Administrative interface                     │   │   │
│   │  └───────────────────────────────────────────────────┘   │   │
│   │                                                          │   │
│   │  ┌───────────────────────────────────────────────────┐   │   │
│   │  │                Worker Threads (N)                 │   │   │
│   │  │  • Connection handling (non-blocking)             │   │   │
│   │  │  • HTTP processing                              │   │   │
│   │  │  • Filter chain execution                       │   │   │
│   │  └───────────────────────────────────────────────────┘   │   │
│   │                                                          │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                   │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                 Envoy Configuration Model                │   │
│   │                                                          │   │
│   │  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌────────┐ │   │
│   │  │ Listener │──▶│  Route   │──▶│  Cluster │──▶│Endpoint│ │   │
│   │  │  :8080   │   │  Match   │   │Discovery │   │  EDS   │ │   │
│   │  └──────────┘   └──────────┘   └──────────┘   └────────┘ │   │
│   │                                                          │   │
│   │  ┌───────────────────────────────────────────────────┐ │   │
│   │  │              HTTP Connection Manager              │ │   │
│   │  │                                                    │ │   │
│   │  │  ┌───────────────────────────────────────────────┐  │ │   │
│   │  │  │           Filter Chain                       │  │ │   │
│   │  │  │  ┌─────────┐┌─────────┐┌─────────┐┌────────┐ │  │ │   │
│   │  │  │  │ Router  ││ RateLimit││   JWT   ││  OAuth │ │  │ │   │
│   │  │  │  │ Filter  ││ Filter  ││ Filter  ││ Filter │ │  │ │   │
│   │  │  │  └─────────┘└─────────┘└─────────┘└────────┘ │  │ │   │
│   │  │  └───────────────────────────────────────────────┘  │ │   │
│   │  │                                                    │ │   │
│   │  └───────────────────────────────────────────────────┘ │   │
│   │                                                          │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                   │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                 XDS Configuration API                    │   │
│   │                                                          │   │
│   │  • LDS (Listener Discovery Service)                     │   │
│   │  • RDS (Route Discovery Service)                          │   │
│   │  • CDS (Cluster Discovery Service)                        │   │
│   │  • EDS (Endpoint Discovery Service)                       │   │
│   │  • SDS (Secret Discovery Service)                         │   │
│   │  • ADS (Aggregated Discovery Service)                     │   │
│   │                                                          │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### HTTP Filter Chain

```
┌─────────────────────────────────────────────────────────────────┐
│                  ENVOY HTTP FILTER CHAIN                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│   Downstream Request                                              │
│        │                                                          │
│        ▼                                                          │
│   ┌─────────────────────────────────────────────────────────┐    │
│   │  1. Downstream HTTP Filter Chain (Decode)               │    │
│   │                                                          │    │
│   │  ┌────────────┐  ┌────────────┐  ┌────────────┐        │    │
│   │  │ Cors Filter│─▶│ Wasm Filter│─▶│ Ext Authz  │        │    │
│   │  │            │  │ (Custom)   │  │ (External) │        │    │
│   │  └────────────┘  └────────────┘  └────────────┘        │    │
│   │        │                                                    │    │
│   │        ▼                                                    │    │
│   │  ┌────────────┐  ┌────────────┣  ┌────────────┐        │    │
│   │  │ Rate Limit │─▶│   Fault    │─▶│ Router     │        │    │
│   │  │  Filter    │  │  Filter    │  │  Filter    │        │    │
│   │  └────────────┘  └────────────┘  └────────────┘        │    │
│   │                                                          │    │
│   └────────────────────────┬────────────────────────────────┘    │
│                            │                                      │
│                            ▼                                      │
│   ┌─────────────────────────────────────────────────────────┐    │
│   │  2. Upstream HTTP Filter Chain (Encode)                 │    │
│   │                                                          │    │
│   │  ┌────────────┐  ┌────────────┐  ┌────────────┐        │    │
│   │  │ Ext Proc   │─▶│   Wasm     │─▶│ Compressor │        │    │
│   │  │ (External) │  │  Filter    │  │  Filter    │        │    │
│   │  └────────────┘  └────────────┘  └────────────┘        │    │
│   │                                                          │    │
│   └────────────────────────┬────────────────────────────────┘    │
│                            │                                      │
│                            ▼                                      │
│   ┌─────────────────────────────────────────────────────────┐    │
│   │  3. Upstream HTTP Filter Chain (Decode Response)        │    │
│   │                                                          │    │
│   │  ┌────────────┐  ┌────────────┐  ┌────────────┐        │    │
│   │  │   Wasm     │─▶│ Ext Proc   │─▶│  Buffer    │        │    │
│   │  │  Filter    │  │ (External) │  │  Filter    │        │    │
│   │  └────────────┘  └────────────┘  └────────────┘        │    │
│   │                                                          │    │
│   └────────────────────────┬────────────────────────────────┘    │
│                            │                                      │
│                            ▼                                      │
│   ┌─────────────────────────────────────────────────────────┐    │
│   │  4. Downstream HTTP Filter Chain (Encode Response)      │    │
│   │                                                          │    │
│   │  ┌────────────┐  ┌────────────┐  ┌────────────┐        │    │
│   │  │   GRPC-Web│─▶│   Wasm     │─▶│  Router    │        │    │
│   │  │   Filter  │  │  Filter    │  │  (Return)  │        │    │
│   │  └────────────┘  └────────────┘  └────────────┘        │    │
│   │                                                          │    │
│   └─────────────────────────────────────────────────────────┘    │
│                                                                   │
│   ┌─────────────────────────────────────────────────────────┐    │
│   │  5. Access Log (after response to client)               │    │
│   │     - File, gRPC, OpenTelemetry                         │    │
│   └─────────────────────────────────────────────────────────┘    │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### Configuration Example

```yaml
# envoy.yaml - Envoy Proxy Configuration for API Gateway

admin:
  address:
    socket_address:
      address: 0.0.0.0
      port_value: 9901
  access_log:
    - name: envoy.access_loggers.stdout
      typed_config:
        "@type": type.googleapis.com/envoy.extensions.access_loggers.stream.v3.StdoutAccessLog

static_resources:
  listeners:
    - name: listener_http
      address:
        socket_address:
          address: 0.0.0.0
          port_value: 80
      filter_chains:
        - filters:
            - name: envoy.filters.network.http_connection_manager
              typed_config:
                "@type": type.googleapis.com/envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager
                stat_prefix: ingress_http
                codec_type: AUTO
                route_config:
                  name: local_route
                  virtual_hosts:
                    - name: redirect_to_https
                      domains: ["*"]
                      routes:
                        - match:
                            prefix: "/"
                          redirect:
                            https_redirect: true
                http_filters:
                  - name: envoy.filters.http.router
                    typed_config:
                      "@type": type.googleapis.com/envoy.extensions.filters.http.router.v3.Router

    - name: listener_https
      address:
        socket_address:
          address: 0.0.0.0
          port_value: 443
      filter_chains:
        - filters:
            - name: envoy.filters.network.http_connection_manager
              typed_config:
                "@type": type.googleapis.com/envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager
                stat_prefix: ingress_https
                codec_type: AUTO
                generate_request_id: true
                preserve_external_request_id: true
                use_remote_address: true
                xff_num_trusted_hops: 1

                access_log:
                  - name: envoy.access_loggers.stdout
                    typed_config:
                      "@type": type.googleapis.com/envoy.extensions.access_loggers.stream.v3.StdoutAccessLog
                      log_format:
                        json_format:
                          timestamp: "%START_TIME%"
                          method: "%REQ(:METHOD)%"
                          path: "%REQ(X-ENVOY-ORIGINAL-PATH?:PATH)%"
                          protocol: "%PROTOCOL%"
                          response_code: "%RESPONSE_CODE%"
                          response_flags: "%RESPONSE_FLAGS%"
                          bytes_received: "%BYTES_RECEIVED%"
                          bytes_sent: "%BYTES_SENT%"
                          duration: "%DURATION%"
                          upstream_service_time: "%RESP(X-ENVOY-UPSTREAM-SERVICE-TIME)%"
                          forwarded_for: "%REQ(X-FORWARDED-FOR)%"
                          user_agent: "%REQ(USER-AGENT)%"
                          request_id: "%REQ(X-REQUEST-ID)%"
                          authority: "%REQ(:AUTHORITY)%"
                          upstream_host: "%UPSTREAM_HOST%"

                route_config:
                  name: api_routes
                  virtual_hosts:
                    - name: portalis_api
                      domains: ["api.portalis.io"]
                      cors:
                        allow_origins:
                          - exact: "https://app.portalis.io"
                          - exact: "https://admin.portalis.io"
                        allow_methods: GET, POST, PUT, DELETE, OPTIONS, PATCH
                        allow_headers:
                          - authorization
                          - content-type
                          - x-request-id
                          - x-correlation-id
                          - x-api-key
                        allow_credentials: true
                        max_age: "3600"
                      routes:
                        # User Service Routes
                        - match:
                            prefix: "/api/v1/users"
                          route:
                            cluster: user_service
                            timeout: 30s
                            retry_policy:
                              retry_on: gateway_error,connect_failure,refused_stream
                              num_retries: 3
                              per_try_timeout: 10s
                              retry_back_off:
                                base_interval: 0.25s
                                max_interval: 1s
                          typed_per_filter_config:
                            envoy.filters.http.ext_authz:
                              "@type": type.googleapis.com/envoy.extensions.filters.http.ext_authz.v3.ExtAuthzPerRoute
                              check_settings:
                                context_extensions:
                                  route: users

                        # Billing Service Routes
                        - match:
                            prefix: "/api/v1/billing"
                          route:
                            cluster: billing_service
                            timeout: 45s
                          typed_per_filter_config:
                            envoy.filters.http.ext_authz:
                              "@type": type.googleapis.com/envoy.extensions.filters.http.ext_authz.v3.ExtAuthzPerRoute
                              disabled: false

                        # Public Routes (no auth)
                        - match:
                            prefix: "/api/v1/public"
                          route:
                            cluster: public_service
                            timeout: 10s
                          typed_per_filter_config:
                            envoy.filters.http.ext_authz:
                              "@type": type.googleapis.com/envoy.extensions.filters.http.ext_authz.v3.ExtAuthzPerRoute
                              disabled: true

                http_filters:
                  # CORS Filter
                  - name: envoy.filters.http.cors
                    typed_config:
                      "@type": type.googleapis.com/envoy.extensions.filters.http.cors.v3.Cors

                  # External Authorization
                  - name: envoy.filters.http.ext_authz
                    typed_config:
                      "@type": type.googleapis.com/envoy.extensions.filters.http.ext_authz.v3.ExtAuthz
                      grpc_service:
                        envoy_grpc:
                          cluster_name: ext_authz
                          timeout: 0.5s
                        initial_metadata:
                          - key: x-request-id
                            value: "%REQ(X-REQUEST-ID)%"
                      include_peer_certificate: true

                  # JWT Authentication
                  - name: envoy.filters.http.jwt_authn
                    typed_config:
                      "@type": type.googleapis.com/envoy.extensions.filters.http.jwt_authn.v3.JwtAuthentication
                      providers:
                        portalis_jwt:
                          remote_jwks:
                            http_uri:
                              uri: http://auth-service.internal:8080/.well-known/jwks.json
                              cluster: auth_service
                              timeout: 5s
                            cache_duration:
                              seconds: 300
                          forward: true
                          forward_payload_header: x-jwt-payload
                          payload_in_metadata: jwt_payload
                      rules:
                        - match:
                            prefix: /api/v1/users
                          requires:
                            provider_name: portalis_jwt
                        - match:
                            prefix: /api/v1/billing
                          requires:
                            provider_name: portalis_jwt
                        - match:
                            prefix: /api/v1/public
                          requires:
                            allow_missing_or_failed: true

                  # Local Rate Limiting
                  - name: envoy.filters.http.local_ratelimit
                    typed_config:
                      "@type": type.googleapis.com/envoy.extensions.filters.http.local_ratelimit.v3.LocalRateLimit
                      stat_prefix: http_local_rate_limiter
                      token_bucket:
                        max_tokens: 1000
                        tokens_per_fill: 100
                        fill_interval: 1s
                      filter_enabled:
                        runtime_key: local_rate_limit_enabled
                        default_value:
                          numerator: 100
                          denominator: HUNDRED
                      filter_enforced:
                        runtime_key: local_rate_limit_enforced
                        default_value:
                          numerator: 100
                          denominator: HUNDRED
                      response_headers_to_add:
                        - append_action: OVERWRITE_IF_EXISTS_OR_ADD
                          header:
                            key: x-local-rate-limit
                            value: 'true'

                  # Global Rate Limiting
                  - name: envoy.filters.http.ratelimit
                    typed_config:
                      "@type": type.googleapis.com/envoy.extensions.filters.http.ratelimit.v3.RateLimit
                      domain: portalis_api
                      rate_limit_service:
                        grpc_service:
                          envoy_grpc:
                            cluster_name: rate_limit_service
                            timeout: 0.2s
                        transport_api_version: V3
                      enable_x_ratelimit_headers: DRAFT_VERSION_03
                      always_consume_default_token_bucket: false

                  # Compression
                  - name: envoy.filters.http.compressor
                    typed_config:
                      "@type": type.googleapis.com/envoy.extensions.filters.http.compressor.v3.Compressor
                      response_direction_config:
                        common_config:
                          min_content_length: 100
                          content_type:
                            - application/json
                            - application/xml
                            - text/html
                            - text/plain
                        compressor_library:
                          name: text_optimized
                          typed_config:
                            "@type": type.googleapis.com/envoy.extensions.compression.gzip.compressor.v3.Gzip
                            memory_level: 3
                            compression_level: COMPRESSION_LEVEL_4
                            compression_strategy: DEFAULT_STRATEGY
                            chunk_size: 4096

                  # Router (must be last)
                  - name: envoy.filters.http.router
                    typed_config:
                      "@type": type.googleapis.com/envoy.extensions.filters.http.router.v3.Router
                      start_child_span: true

          transport_socket:
            name: envoy.transport_sockets.tls
            typed_config:
              "@type": type.googleapis.com/envoy.extensions.transport_sockets.tls.v3.DownstreamTlsContext
              common_tls_context:
                tls_certificates:
                  - certificate_chain:
                      filename: /etc/envoy/certs/portalis.crt
                    private_key:
                      filename: /etc/envoy/certs/portalis.key
                alpn_protocols: ["h2", "http/1.1"]

  clusters:
    - name: user_service
      connect_timeout: 5s
      type: STRICT_DNS
      lb_policy: LEAST_REQUEST
      circuit_breakers:
        thresholds:
          - priority: DEFAULT
            max_connections: 1000
            max_pending_requests: 1000
            max_requests: 1000
            max_retries: 3
      health_checks:
        - timeout: 5s
          interval: 10s
          unhealthy_threshold: 3
          healthy_threshold: 2
          http_health_check:
            path: /health
            expected_statuses:
              start: 200
              end: 200
      load_assignment:
        cluster_name: user_service
        endpoints:
          - lb_endpoints:
              - endpoint:
                  address:
                    socket_address:
                      address: user-api-1.internal
                      port_value: 8080
                  health_check_config:
                    port_value: 8080
                metadata:
                  filter_metadata:
                    envoy.lb:
                      canary: false
              - endpoint:
                  address:
                    socket_address:
                      address: user-api-2.internal
                      port_value: 8080
                  health_check_config:
                    port_value: 8080
                metadata:
                  filter_metadata:
                    envoy.lb:
                      canary: false
              - endpoint:
                  address:
                    socket_address:
                      address: user-api-canary.internal
                      port_value: 8080
                  health_check_config:
                    port_value: 8080
                metadata:
                  filter_metadata:
                    envoy.lb:
                      canary: true
                      version: "2.0.0"
      common_http_protocol_options:
        idle_timeout: 300s
      upstream_connection_options:
        tcp_keepalive:
          keepalive_time: 300
          keepalive_interval: 75
          keepalive_probes: 9

    - name: billing_service
      connect_timeout: 5s
      type: STRICT_DNS
      lb_policy: ROUND_ROBIN
      health_checks:
        - timeout: 5s
          interval: 10s
          unhealthy_threshold: 3
          healthy_threshold: 2
          http_health_check:
            path: /health
      load_assignment:
        cluster_name: billing_service
        endpoints:
          - lb_endpoints:
              - endpoint:
                  address:
                    socket_address:
                      address: billing-api-1.internal
                      port_value: 8080
              - endpoint:
                  address:
                    socket_address:
                      address: billing-api-2.internal
                      port_value: 8080

    - name: ext_authz
      connect_timeout: 1s
      type: STRICT_DNS
      lb_policy: ROUND_ROBIN
      http2_protocol_options: {}
      load_assignment:
        cluster_name: ext_authz
        endpoints:
          - lb_endpoints:
              - endpoint:
                  address:
                    socket_address:
                      address: auth-service.internal
                      port_value: 8080

    - name: rate_limit_service
      connect_timeout: 1s
      type: STRICT_DNS
      lb_policy: ROUND_ROBIN
      http2_protocol_options: {}
      load_assignment:
        cluster_name: rate_limit_service
        endpoints:
          - lb_endpoints:
              - endpoint:
                  address:
                    socket_address:
                      address: rate-limit-service.internal
                      port_value: 8080

    - name: auth_service
      connect_timeout: 5s
      type: STRICT_DNS
      lb_policy: ROUND_ROBIN
      load_assignment:
        cluster_name: auth_service
        endpoints:
          - lb_endpoints:
              - endpoint:
                  address:
                    socket_address:
                      address: auth-service.internal
                      port_value: 8080

  # Rate limit configuration
  rate_limit_configs:
    - name: portalis_ratelimit
      domain: portalis_api
      descriptors:
        - key: generic_key
          value: default
          rate_limit:
            unit: second
            requests_per_unit: 1000
        - key: remote_address
          rate_limit:
            unit: minute
            requests_per_unit: 100

# Dynamic configuration via ADS
dynamic_resources:
  cds_config:
    resource_api_version: V3
    api_config_source:
      api_type: GRPC
      transport_api_version: V3
      grpc_services:
        - envoy_grpc:
            cluster_name: xds_cluster
      set_node_on_first_message_only: true
  lds_config:
    resource_api_version: V3
    api_config_source:
      api_type: GRPC
      transport_api_version: V3
      grpc_services:
        - envoy_grpc:
            cluster_name: xds_cluster
      set_node_on_first_message_only: true
```

### Key Features

| Feature | Description | Envoy Filter |
|---------|-------------|--------------|
| **XDS API** | Dynamic configuration | LDS, RDS, CDS, EDS |
| **HTTP Filters** | Modular request processing | 30+ built-in filters |
| **gRPC** | First-class gRPC support | gRPC-Web, gRPC-JSON transcoding |
| **Load Balancing** | Advanced algorithms | Least request, ring hash, Maglev |
| **Circuit Breaker** | Automatic failure detection | Outlier detection |
| **Observability** | Distributed tracing | OpenTelemetry, Zipkin, Jaeger |
| **WASM** | WebAssembly extensions | Proxy-WASM SDK |
| **Rate Limiting** | Token bucket algorithm | Local and global rate limiting |
| **Authentication** | JWT, OAuth2, OIDC | JWT Authn filter, Ext Authz |
| **Compression** | gzip, brotli | Compressor filter |

### Performance Characteristics

```
┌─────────────────────────────────────────────────────────────────┐
│                  ENVOY PERFORMANCE PROFILE                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│   Latency (p99): 0.5-2ms (proxy only)                             │
│                  2-8ms (with auth + rate limiting)              │
│                  5-15ms (with WASM filter)                        │
│                                                                   │
│   Throughput:    100,000+ RPS per node                             │
│                                                                   │
│   Memory:        ~150MB base + connections                        │
│                                                                   │
│   Scalability:   Horizontal via control plane                      │
│                  Stateless data plane                              │
│                                                                   │
│   Connection Handling: Event-driven (libevent)                    │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### Pros and Cons

**Pros:**
- Modern architecture with XDS API
- Excellent observability features
- Strong WASM support for extensions
- Language-agnostic (control plane in any language)
- Best-in-class load balancing
- Active CNCF project

**Cons:**
- Complex configuration (steep learning curve)
- Resource intensive (compared to raw NGINX)
- Requires control plane for dynamic features
- Configuration validation can be challenging

---

## AWS API Gateway

### Overview

AWS API Gateway is a fully managed service that makes it easy for developers to create, publish, maintain, monitor, and secure APIs at any scale.

```
┌─────────────────────────────────────────────────────────────────┐
│                   AWS API GATEWAY ARCHITECTURE                   │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                    AWS API Gateway                       │   │
│   │                                                          │   │
│   │  ┌─────────────────────────────────────────────────────┐  │   │
│   │  │                    REST API                         │  │   │
│   │  │  • HTTP/REST endpoints                             │  │   │
│   │  │  • Request/response transformation                 │  │   │
│   │  │  • Caching with CloudFront                        │  │   │
│   │  │  • Usage plans and API keys                        │  │   │
│   │  │  • SDK generation                                 │  │   │
│   │  └─────────────────────────────────────────────────────┘  │   │
│   │                                                          │   │
│   │  ┌─────────────────────────────────────────────────────┐  │   │
│   │  │                   HTTP API                           │  │   │
│   │  │  • Lower latency, lower cost                       │  │   │
│   │  │  • OIDC and OAuth 2.0 support                      │  │   │
│   │  │  • AWS IAM and Lambda authorizers                 │  │   │
│   │  │  • CORS support                                   │  │   │
│   │  └─────────────────────────────────────────────────────┘  │   │
│   │                                                          │   │
│   │  ┌─────────────────────────────────────────────────────┐  │   │
│   │  │                 WebSocket API                       │  │   │
│   │  │  • Real-time bidirectional communication           │  │   │
│   │  │  • AWS IoT Core integration                        │  │   │
│   │  │  • Connection management                          │  │   │
│   │  └─────────────────────────────────────────────────────┘  │   │
│   │                                                          │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                   │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │              Integration Types                          │   │
│   │                                                          │   │
│   │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌────────┐  │   │
│   │  │ Lambda   │  │  HTTP    │  │  AWS     │  │ Mock   │  │   │
│   │  │ Function │  │  Proxy   │  │  Service │  │        │  │   │
│   │  └──────────┘  └──────────┘  └──────────┘  └────────┘  │   │
│   │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌────────┐  │   │
│   │  │ EventBridge││  SQS     │  │  Step    │  │ VPC    │  │   │
│   │  │          │  │          │  │ Functions│  │ Link   │  │   │
│   │  └──────────┘  └──────────┘  └──────────┘  └────────┘  │   │
│   │                                                          │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### Configuration Examples

```yaml
# AWS API Gateway OpenAPI (Swagger) Definition
openapi: "3.0.1"
info:
  title: "Portalis API"
  version: "1.0.0"
  description: "Portalis API Gateway Definition"

servers:
  - url: https://api.portalis.io/v1

paths:
  /users:
    get:
      summary: List users
      operationId: listUsers
      security:
        - portalisAuthorizer: []
      x-amazon-apigateway-integration:
        type: http_proxy
        httpMethod: GET
        uri: http://user-service.internal:8080/users
        connectionType: VPC_LINK
        connectionId: "${vpc_link_id}"
        requestParameters:
          integration.request.header.X-Request-ID: "context.requestId"
        responses:
          default:
            statusCode: "200"
            responseParameters:
              method.response.header.Access-Control-Allow-Origin: "'*'"
      responses:
        "200":
          description: "Success"
          headers:
            Access-Control-Allow-Origin:
              schema:
                type: "string"
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/UserList"

    post:
      summary: Create user
      operationId: createUser
      security:
        - portalisAuthorizer: []
      x-amazon-apigateway-integration:
        type: http_proxy
        httpMethod: POST
        uri: http://user-service.internal:8080/users
        connectionType: VPC_LINK
        connectionId: "${vpc_link_id}"
      responses:
        "201":
          description: "Created"

  /billing/{id}:
    get:
      summary: Get billing info
      parameters:
        - name: id
          in: path
          required: true
          schema:
            type: string
      security:
        - portalisAuthorizer: []
      x-amazon-apigateway-request-validator: "params-only"
      x-amazon-apigateway-integration:
        type: http_proxy
        httpMethod: GET
        uri: http://billing-service.internal:8080/billing/{id}
        connectionType: VPC_LINK
        connectionId: "${vpc_link_id}"
        requestParameters:
          integration.request.path.id: "method.request.path.id"
          integration.request.header.X-API-Key: "method.request.header.X-API-Key"
        cacheKeyParameters:
          - "method.request.path.id"
        requestTemplates:
          application/json: |
            {
              "userId": "$input.params('id')",
              "requestId": "$context.requestId"
            }
        responses:
          "200":
            statusCode: "200"
            responseTemplates:
              application/json: |
                #set($inputRoot = $input.path('$'))
                {
                  "billingId": "$inputRoot.id",
                  "amount": $inputRoot.amount,
                  "currency": "$inputRoot.currency"
                }
      responses:
        "200":
          description: "Success"

components:
  schemas:
    UserList:
      type: array
      items:
        $ref: "#/components/schemas/User"
    User:
      type: object
      properties:
        id:
          type: string
        email:
          type: string
        name:
          type: string

  securitySchemes:
    portalisAuthorizer:
      type: apiKey
      name: Authorization
      in: header
      x-amazon-apigateway-authtype: custom
      x-amazon-apigateway-authorizer:
        authorizerUri: "arn:aws:apigateway:${region}:lambda:path/2015-03-31/functions/${authorizer_lambda_arn}/invocations"
        authorizerResultTtlInSeconds: 300
        identitySource: "method.request.header.Authorization"
        type: request

x-amazon-apigateway-request-validators:
  params-only:
    validateRequestParameters: true
    validateRequestBody: false

x-amazon-apigateway-gateway-responses:
  DEFAULT_4XX:
    responseParameters:
      gatewayresponse.header.Access-Control-Allow-Origin: "'*'"
    responseTemplates:
      application/json: |
        {"message": "Bad Request", "requestId": "$context.requestId"}
  DEFAULT_5XX:
    responseParameters:
      gatewayresponse.header.Access-Control-Allow-Origin: "'*'"
    responseTemplates:
      application/json: |
        {"message": "Internal Server Error", "requestId": "$context.requestId"}
```

```python
# AWS CDK API Gateway Configuration
from aws_cdk import (
    Stack,
    aws_apigateway as apigw,
    aws_lambda as lambda_,
    aws_ec2 as ec2,
    aws_logs as logs,
    Duration,
)
from constructs import Construct

class PortalisApiGatewayStack(Stack):
    def __init__(self, scope: Construct, construct_id: str, **kwargs) -> None:
        super().__init__(scope, construct_id, **kwargs)

        # VPC Link for private integration
        vpc = ec2.Vpc.from_lookup(self, "Vpc", vpc_id="vpc-xxxxx")
        vpc_link = apigw.VpcLink(self, "PortalisVpcLink",
            vpc_link_name="portalis-vpc-link",
            targets=[
                # NLB targets
            ]
        )

        # Custom authorizer Lambda
        authorizer_lambda = lambda_.Function(self, "PortalisAuthorizer",
            runtime=lambda_.Runtime.PYTHON_3_11,
            handler="index.handler",
            code=lambda_.Code.from_asset("lambda/authorizer"),
            timeout=Duration.seconds(5),
        )

        custom_authorizer = apigw.RequestAuthorizer(self, "PortalisAuthorizer",
            handler=authorizer_lambda,
            identity_sources=[apigw.IdentitySource.header("Authorization")],
            results_cache_ttl=Duration.minutes(5),
        )

        # API Definition
        api = apigw.RestApi(self, "PortalisApi",
            rest_api_name="Portalis API",
            description="Portalis API Gateway",
            deploy_options=apigw.StageOptions(
                stage_name="prod",
                throttling_rate_limit=1000,
                throttling_burst_limit=200,
                caching_enabled=True,
                cache_ttl=Duration.seconds(300),
                data_trace_enabled=True,
                logging_level=apigw.MethodLoggingLevel.INFO,
                access_log_destination=apigw.LogGroupLogDestination(
                    logs.LogGroup(self, "PortalisApiLogs")
                ),
                metrics_enabled=True,
            ),
            default_cors_preflight_options=apigw.CorsOptions(
                allow_origins=apigw.Cors.ALL_ORIGINS,
                allow_methods=apigw.Cors.ALL_METHODS,
                allow_headers=["Authorization", "Content-Type", "X-Request-ID"],
            ),
            endpoint_types=[apigw.EndpointType.REGIONAL],
        )

        # Usage Plans
        free_tier_plan = api.add_usage_plan("FreeTier",
            name="Free Tier",
            throttle=apigw.ThrottleSettings(
                rate_limit=100,
                burst_limit=50,
            ),
            quota=apigw.QuotaSettings(
                limit=10000,
                period=apigw.Period.MONTH,
            ),
        )

        pro_tier_plan = api.add_usage_plan("ProTier",
            name="Pro Tier",
            throttle=apigw.ThrottleSettings(
                rate_limit=1000,
                burst_limit=500,
            ),
            quota=apigw.QuotaSettings(
                limit=100000,
                period=apigw.Period.MONTH,
            ),
        )

        # API Key
        api_key = api.add_api_key("PortalApiKey",
            api_key_name="portal-client-key",
        )
        pro_tier_plan.add_api_key(api_key)

        # Resources and Methods
        users = api.root.add_resource("users")
        users.add_method("GET",
            integration=apigw.HttpIntegration(
                url="http://user-service:8080/users",
                http_method="GET",
                options=apigw.IntegrationOptions(
                    connection_type=apigw.ConnectionType.VPC_LINK,
                    vpc_link=vpc_link,
                ),
            ),
            authorizer=custom_authorizer,
            method_responses=[
                apigw.MethodResponse(status_code="200"),
            ],
        )

        # WAF Integration
        # ... (AWS WAF WebACL association)
```

### Key Features

| Feature | REST API | HTTP API | WebSocket API |
|---------|----------|----------|---------------|
| **Latency** | Higher | Lower (lower cost) | Real-time |
| **Caching** | CloudFront | No | No |
| **Request Validation** | Yes | Limited | No |
| **Response Transformation** | VTL templates | Limited | No |
| **Private Integration** | VPC Link | VPC Link | No |
| **Usage Plans** | Yes | Yes | No |
| **SDK Generation** | Yes | No | No |
| **Authorization** | IAM, Lambda, Cognito, API Key | IAM, Lambda, OIDC, OAuth2 | Lambda |

### Pricing Comparison

```
┌─────────────────────────────────────────────────────────────────┐
│                 AWS API GATEWAY PRICING                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│   REST API:                                                       │
│   • $3.50 per million API calls received                        │
│   • $0.09/GB data transfer out                                   │
│   • Caching: from $0.02/hour (0.5GB)                            │
│                                                                   │
│   HTTP API:                                                       │
│   • $1.00 per million API calls received                        │
│   • $0.09/GB data transfer out                                   │
│   • No caching                                                     │
│                                                                   │
│   WebSocket API:                                                  │
│   • $1.00 per million messages (first 1B)                        │
│   • $0.25 per million connection minutes                         │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

---

## Azure API Management

### Overview

Azure API Management is a hybrid, multicloud management platform for APIs across all environments. It provides a comprehensive set of capabilities for publishing, securing, transforming, maintaining, and monitoring APIs.

```
┌─────────────────────────────────────────────────────────────────┐
│              AZURE API MANAGEMENT ARCHITECTURE                   │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │              Azure API Management (APIM)                │   │
│   │                                                          │   │
│   │  ┌─────────────────┐  ┌─────────────────────────────┐   │   │
│   │  │   Management    │  │          Gateway             │   │   │
│   │  │     Plane       │  │                             │   │   │
│   │  │                 │  │  ┌─────────────────────┐    │   │   │
│   │  │ • Portal        │  │  │   Request Router    │    │   │   │
│   │  │ • Analytics     │  │  │                     │    │   │   │
│   │  │ • Policies      │─▶│  │ • URL routing       │    │   │   │
│   │  │ • Products      │  │  │ • Auth validation   │    │   │   │
│   │  │ • Users         │  │  │ • Rate limiting     │    │   │   │
│   │  │                 │  │  │ • Caching             │    │   │   │
│   │  └─────────────────┘  │  │ • Transformation      │    │   │   │
│   │                       │  └─────────────────────┘    │   │   │
│   │  ┌─────────────────┐  │                             │   │   │
│   │  │   Developer     │  │  ┌─────────────────────┐    │   │   │
│   │  │     Portal      │  │  │  Backend Services   │    │   │   │
│   │  │                 │  │  │                     │    │   │   │
│   │  │ • API docs      │  │  │ • HTTP/REST         │    │   │   │
│   │  │ • Interactive   │  │  │ • SOAP              │    │   │   │
│   │  │   console       │  │  │ • GraphQL           │    │   │   │
│   │  │ • Code samples  │  │  │ • Function App      │    │   │   │
│   │  │ • Analytics     │  │  │ • Logic App         │    │   │   │
│   │  └─────────────────┘  │  └─────────────────────┘    │   │   │
│   │                       │                             │   │   │
│   └───────────────────────┴─────────────────────────────┘   │
│                                                                   │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                    Policy Pipeline                      │   │
│   │                                                          │   │
│   │  Inbound ▶ Backend ▶ Outbound                             │   │
│   │                                                          │   │
│   │  ┌──────────┐  ┌──────────┐  ┌──────────┐               │   │
│   │  │  CORS    │  │  Cache   │  │  Set     │               │   │
│   │  │  Auth    │  │  Lookup  │  │  Headers │               │   │
│   │  │  Rate    │─▶│  Route   │─▶│  Mask    │               │   │
│   │  │  Limit   │  │  To      │  │  URLs    │               │   │
│   │  │  Validate│  │  Backend │  │          │               │   │
│   │  └──────────┘  └──────────┘  └──────────┘               │   │
│   │                                                          │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### Policy Configuration

```xml
<!-- Azure API Management Policies -->
<policies>
    <inbound>
        <!-- Base policy reference -->
        <base />

        <!-- CORS Configuration -->
        <cors allow-credentials="true">
            <allowed-origins>
                <origin>https://app.portalis.io</origin>
                <origin>https://admin.portalis.io</origin>
            </allowed-origins>
            <allowed-methods>
                <method>GET</method>
                <method>POST</method>
                <method>PUT</method>
                <method>DELETE</method>
                <method>OPTIONS</method>
            </allowed-methods>
            <allowed-headers>
                <header>Authorization</header>
                <header>Content-Type</header>
                <header>X-Request-ID</header>
                <header>X-API-Key</header>
                <header>X-Correlation-ID</header>
            </allowed-headers>
            <expose-headers>
                <header>X-Request-ID</header>
                <header>X-RateLimit-Remaining</header>
            </expose-headers>
        </cors>

        <!-- Rate Limiting by Key -->
        <rate-limit-by-key calls="100" renewal-period="60"
            counter-key="@(context.Request.Headers.GetValueOrDefault("X-API-Key", ""))"
            increment-condition="@(context.Response.StatusCode < 500)"
            remaining-calls-header-name="X-RateLimit-Remaining"
            total-calls-header-name="X-RateLimit-Limit" />

        <!-- Conditional Rate Limiting -->
        <choose>
            <when condition="@(context.Request.Headers.GetValueOrDefault("X-Tier", "free") == "free")">
                <rate-limit calls="100" renewal-period="60" />
            </when>
            <when condition="@(context.Request.Headers.GetValueOrDefault("X-Tier", "free") == "pro")">
                <rate-limit calls="1000" renewal-period="60" />
            </when>
            <otherwise>
                <rate-limit calls="5000" renewal-period="60" />
            </otherwise>
        </choose>

        <!-- JWT Validation -->
        <validate-jwt
            header-name="Authorization"
            failed-validation-httpcode="401"
            failed-validation-error-message="Unauthorized"
            require-expiration-time="true"
            require-scheme="Bearer"
            require-signed-tokens="true">
            <openid-config url="https://auth.portalis.io/.well-known/openid-configuration" />
            <audiences>
                <audience>portalis-api</audience>
            </audiences>
            <issuers>
                <issuer>https://auth.portalis.io</issuer>
            </issuers>
            <required-claims>
                <claim name="scope" match="any">
                    <value>api:read</value>
                    <value>api:write</value>
                </claim>
            </required-claims>
        </validate-jwt>

        <!-- Cache Lookup -->
        <cache-lookup vary-by-developer="false" vary-by-developer-groups="false"
            vary-by-header="X-API-Key" vary-by-query-parameter=""
            downstream-caching-type="none">
            <vary-by-header>Accept</vary-by-header>
            <vary-by-header>Accept-Charset</vary-by-header>
            <vary-by-header>Accept-Encoding</vary-by-header>
        </cache-lookup>

        <!-- Set Headers for Backend -->
        <set-header name="X-Request-ID" exists-action="override">
            <value>@(Guid.NewGuid().ToString())</value>
        </set-header>
        <set-header name="X-Forwarded-Host" exists-action="override">
            <value>@(context.Request.OriginalUrl.Host)</value>
        </set-header>

        <!-- URL Rewriting -->
        <rewrite-uri template="/api/v2/{path}" />

        <!-- Backend Authentication -->
        <authentication-managed-identity resource="https://management.azure.com/" />
    </inbound>

    <backend>
        <base />
        <!-- Forward request to backend -->
        <forward-request follow-redirects="true" />
    </backend>

    <outbound>
        <base />

        <!-- Cache Store -->
        <cache-store duration="300" />

        <!-- Mask URLs in Response -->
        <redirect-content-urls />

        <!-- Set Response Headers -->
        <set-header name="X-Request-ID" exists-action="override">
            <value>@(context.Request.Headers.GetValueOrDefault("X-Request-ID", ""))</value>
        </set-header>

        <!-- Transform JSON Response -->
        <set-body template="liquid">
        {
            "data": {{body}},
            "meta": {
                "requestId": "{{context.RequestId}}",
                "timestamp": "{{context.Timestamp}}"
            }
        }
        </set-body>

        <!-- Error Handling -->
        <on-error>
            <base />
            <set-status code="500" reason="Internal Server Error" />
            <set-header name="Content-Type" exists-action="override">
                <value>application/json</value>
            </set-header>
            <set-body>
            {
                "error": "An error occurred processing your request",
                "requestId": "@(context.RequestId)"
            }
            </set-body>
        </on-error>
    </outbound>

    <on-error>
        <base />
        <!-- Error handling policies -->
    </on-error>
</policies>
```

### Terraform Configuration

```hcl
# Azure API Management Terraform Configuration

resource "azurerm_api_management" "portalis" {
  name                = "portalis-apim"
  location            = azurerm_resource_group.portalis.location
  resource_group_name = azurerm_resource_group.portalis.name
  publisher_name      = "Portalis Inc"
  publisher_email     = "admin@portalis.io"

  sku_name = "Premium_2"  # For high availability

  identity {
    type = "SystemAssigned"
  }

  policy {
    xml_content = <<XML
    <policies>
      <inbound>
        <cors allow-credentials="true">
          <allowed-origins>
            <origin>https://app.portalis.io</origin>
          </allowed-origins>
          <allowed-methods>
            <method>GET</method>
            <method>POST</method>
          </allowed-methods>
        </cors>
      </inbound>
    </policies>
    XML
  }
}

resource "azurerm_api_management_api" "users" {
  name                = "users-api"
  resource_group_name = azurerm_resource_group.portalis.name
  api_management_name = azurerm_api_management.portalis.name
  revision            = "1"
  display_name        = "Users API"
  path                = "users"
  protocols           = ["https"]
  service_url         = "https://user-service.internal:8080"

  import {
    content_format = "openapi+json"
    content_value  = file("${path.module}/openapi/users.json")
  }
}

resource "azurerm_api_management_product" "standard" {
  product_id            = "standard"
  api_management_name     = azurerm_api_management.portalis.name
  resource_group_name     = azurerm_resource_group.portalis.name
  display_name            = "Standard"
  subscription_required = true
  approval_required       = false
  published               = true
}

resource "azurerm_api_management_product_api" "users_standard" {
  product_id          = azurerm_api_management_product.standard.product_id
  api_name            = azurerm_api_management_api.users.name
  api_management_name = azurerm_api_management.portalis.name
  resource_group_name = azurerm_resource_group.portalis.name
}

resource "azurerm_api_management_named_value" "jwt_secret" {
  name                = "jwt-validation-key"
  resource_group_name = azurerm_resource_group.portalis.name
  api_management_name = azurerm_api_management.portalis.name
  display_name        = "JWT Validation Key"
  value               = var.jwt_secret
  secret              = true
}
```

### Key Features

| Feature | Description | Policy Support |
|---------|-------------|----------------|
| **Products** | API grouping and packaging | Full |
| **Subscriptions** | API key management | Full |
| **Policies** | XML-based transformation | 50+ policies |
| **Developer Portal** | Customizable API documentation | Built-in |
| **Analytics** | Usage and performance metrics | Built-in |
| **Caching** | Response caching | Built-in |
| **VNET Integration** | Private network access | Premium SKU |
| **Multi-region** | Geographic distribution | Premium SKU |
| **Self-hosted Gateway** | On-premise deployment | Premium SKU |

---

## Cloudflare API Gateway

### Overview

Cloudflare API Gateway provides API security and management at the edge of Cloudflare's global network, offering low-latency access worldwide.

```
┌─────────────────────────────────────────────────────────────────┐
│              CLOUDFLARE API GATEWAY ARCHITECTURE               │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │              Cloudflare Global Network                  │   │
│   │              (300+ Cities, 100+ Countries)              │   │
│   │                                                          │   │
│   │  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐    │   │
│   │  │   SF    │  │  NYC    │  │  LDN    │  │  TYO    │    │   │
│   │  │  Edge   │  │  Edge   │  │  Edge   │  │  Edge   │    │   │
│   │  └────┬────┘  └────┬────┘  └────┬────┘  └────┬────┘    │   │
│   │       │            │            │            │          │   │
│   │       └────────────┴────────────┴────────────┘          │   │
│   │                    │                                    │   │
│   │                    ▼                                    │   │
│   │  ┌─────────────────────────────────────────────────────┐ │   │
│   │  │              API Gateway Features                    │ │   │
│   │  │                                                     │ │   │
│   │  │  • Discovery and Schema Validation                 │ │   │
│   │  │  • Authentication (mTLS, JWT, API Keys)            │ │   │
│   │  │  • Rate Limiting (Token bucket, concurrent)        │ │   │
│   │  │  • Threat Protection (WAF, DDoS)                   │ │   │
│   │  │  • Logging and Analytics                           │ │   │
│   │  └─────────────────────────────────────────────────────┘ │   │
│   │                                                          │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### Configuration Example

```yaml
# cloudflare-api-gateway.yaml
# Cloudflare API Shield and Gateway Rules

# Zone-level API Gateway settings
api_gateway_settings:
  zone_id: "${ZONE_ID}"

  # API Discovery
  api_discovery:
    enabled: true
    sampling_rate: 1.0
    endpoint_learning: true
    schema_learning: true

  # Schema Validation
  schema_validation:
    enabled: true
    default_action: block
    fallback_action: log
    schemas:
      - name: "users-api"
        source: "https://api.portalis.io/openapi/users.json"
        validation_scope:
          - request_body
          - response_body
          - query_parameters
          - headers

  # API Shield (Security)
  api_shield:
    enabled: true
    auth_id_characteristics:
      - name: "API Key"
        type: "header"
        value: "X-API-Key"
      - name: "JWT Subject"
        type: "jwt_claim"
        value: "sub"
    threshold_sensitivity: "medium"

  # Rate Limiting Rules
  rate_limiting:
    - name: "general-api-limit"
      description: "General API rate limit"
      expression: "http.host eq \"api.portalis.io\""
      action: block
      ratelimit:
        characteristics:
          - "cf.colo.id"
          - "ip.src"
        period: 60
        requests_per_period: 1000
        mitigation_timeout: 600

    - name: "sensitive-endpoint-limit"
      description: "Strict limit for sensitive endpoints"
      expression: |
        http.host eq "api.portalis.io" and
        (http.request.uri.path contains "/billing" or
         http.request.uri.path contains "/admin")
      action: block
      ratelimit:
        characteristics:
          - "ip.src"
        period: 60
        requests_per_period: 100
        mitigation_timeout: 600

    - name: "authenticated-user-limit"
      description: "Per-user rate limit"
      expression: "http.host eq \"api.portalis.io\""
      action: throttle
      ratelimit:
        characteristics:
          - "http.request.headers[\"x-api-key\"]"
        period: 60
        requests_per_period: 500
        mitigation_timeout: 60

  # Sequence Mitigation (API Abuse)
  sequence_mitigation:
    enabled: true
    rules:
      - name: "abnormal-sequence"
        description: "Detect abnormal API call sequences"
        threshold: 10
        window: 300

  # Token Configuration
  token_configuration:
    token_type: jwt
    jwt:
      jwks_uri: "https://auth.portalis.io/.well-known/jwks.json"
      claims:
        - name: "sub"
          use_for_rate_limiting: true
        - name: "tenant_id"
          use_for_routing: true

# Transform Rules for request/response modification
transform_rules:
  - name: "add-request-id"
    expression: "true"
    action:
      - operation: "set_header"
        name: "X-Request-ID"
        value: "${cf.request_id}"

  - name: "sanitize-response"
    expression: "http.host eq \"api.portalis.io\""
    action:
      - operation: "remove_header"
        name: "X-Powered-By"
      - operation: "remove_header"
        name: "Server"

# Cache Rules for API responses
cache_rules:
  - name: "cache-public-endpoints"
    expression: |
      http.host eq "api.portalis.io" and
      http.request.uri.path contains "/public"
    action:
      cache: true
      ttl: 300
      stale_while_revalidate: 60

  - name: "no-cache-sensitive"
    expression: |
      http.host eq "api.portalis.io" and
      (http.request.uri.path contains "/billing" or
       http.request.uri.path contains "/users/me")
    action:
      cache: false
```

### Key Features

| Feature | Description | Edge Location |
|---------|-------------|---------------|
| **API Discovery** | Automatic API endpoint discovery | Edge |
| **Schema Validation** | OpenAPI/Swagger validation | Edge |
| **Sequence Protection** | Abuse pattern detection | Edge |
| **mTLS** | Client certificate auth | Edge |
| **Rate Limiting** | Advanced rate limiting | Edge |
| **DDoS Protection** | Automatic DDoS mitigation | Edge |
| **Caching** | Edge caching | Edge |
| **Bot Management** | Bot detection and mitigation | Edge |

---

## Comparison Matrix

### Feature Comparison

```
┌─────────────────────────┬──────────┬────────────┬─────────┬─────────┬────────┬─────────┬─────────┐
│        Feature          │   Kong   │ Ambassador │ Traefik │  NGINX  │ Envoy  │ AWS APIG│ Azure   │
│                         │          │ Edge Stack │         │  Plus   │        │  ateway │ APIM    │
├─────────────────────────┼──────────┼────────────┼─────────┼─────────┼────────┼─────────┼─────────┤
│ Open Source             │    ✓     │     ✓      │    ✓    │    ~    │   ✓    │    ✗    │    ✗    │
│ Kubernetes Native       │    ✓     │     ✓      │    ✓    │    ~    │   ~    │    ✗    │    ✗    │
│ Hot Reload              │    ✓     │     ✓      │    ✓    │    ~    │   ✓    │    ✓    │    ✓    │
│ Plugin System           │   Lua    │   Envoy    │  Go/CRD │   NJS   │  WASM  │    ✗    │   XML   │
│ Service Discovery       │   DNS    │    K8s     │  Multi  │  DNS    │   XDS  │   AWS   │  Azure  │
│ Rate Limiting           │    ✓     │     ✓      │    ✓    │    ✓    │   ✓    │    ✓    │    ✓    │
│ JWT Validation          │    ✓     │     ✓      │    ✓    │    ~    │   ✓    │    ✓    │    ✓    │
│ OAuth2/OIDC             │    ✓     │     ✓      │    ~    │    ~    │   ✓    │    ✓    │    ✓    │
│ mTLS                    │    ✓     │     ✓      │    ✓    │    ✓    │   ✓    │    ✓    │    ✓    │
│ Request Transform       │    ✓     │     ✓      │    ✓    │    ✓    │   ✓    │    ✓    │    ✓    │
│ Response Transform      │    ✓     │     ✓      │    ~    │    ✓    │   ~    │    ✓    │    ✓    │
│ Circuit Breaker         │    ✓     │     ✓      │    ✓    │    ~    │   ✓    │    ✗    │    ✗    │
│ Retry Logic             │    ✓     │     ✓      │    ✓    │    ✓    │   ✓    │    ✓    │    ✗    │
│ Load Balancing          │    ✓     │     ✓      │    ✓    │    ✓    │   ✓    │    ~    │    ~    │
│ WebSocket Support       │    ✓     │     ✓      │    ✓    │    ✓    │   ✓    │    ✓    │    ✓    │
│ gRPC Support            │    ✓     │     ✓      │    ✓    │    ~    │   ✓    │    ~    │    ✓    │
│ GraphQL                 │    ✓     │     ~      │    ~    │    ✗    │   ~    │    ✗    │    ~    │
│ Caching                 │    ✓     │     ✓      │    ~    │    ✓    │   ~    │    ✓    │    ✓    │
│ Developer Portal        │    $     │     ✓      │    ✗    │    ✗    │   ✗    │    ✗    │    ✓    │
│ Prometheus Metrics      │    ✓     │     ✓      │    ✓    │    ✓    │   ✓    │    ✗    │    ~    │
│ Distributed Tracing     │    ✓     │     ✓      │    ✓    │    ~    │   ✓    │    ~    │    ~    │
│ Multi-tenancy           │    ✓     │     ~      │    ~    │    ~    │   ✓    │    ~    │    ✓    │
│ Custom Auth             │    ✓     │     ✓      │    ✓    │    ✓    │   ✓    │    ✓    │    ✓    │
│ API Versioning          │    ✓     │     ✓      │    ✓    │    ✓    │   ✓    │    ✓    │    ✓    │
│ Traffic Splitting       │    ✓     │     ✓      │    ✓    │    ✓    │   ✓    │    ✗    │    ✗    │
└─────────────────────────┴──────────┴────────────┴─────────┴─────────┴────────┴─────────┴─────────┘

Legend: ✓ = Full Support, ~ = Partial/Limited, ✗ = Not Available, $ = Enterprise Only
```

### Performance Comparison

```
┌─────────────────────────┬──────────┬────────────┬─────────┬─────────┬────────┬─────────┬─────────┐
│       Metric            │   Kong   │ Ambassador │ Traefik │  NGINX  │ Envoy  │ AWS APIG│ Azure   │
│                         │          │ Edge Stack │         │  Plus   │        │  ateway │ APIM    │
├─────────────────────────┼──────────┴────────────┴─────────┴─────────┴────────┴─────────┴─────────┤
│ Base Latency (p99)      │   1-5ms   │   1-3ms    │ 0.5-2ms │ 0.1-1ms │ 0.5-2ms│  10-50ms│  20-100 │
│ With Auth (p99)         │   5-15ms  │   5-10ms   │  2-8ms  │  1-5ms  │  2-8ms │  20-80ms│  30-120 │
│ Max Throughput (RPS)    │  50,000+  │  100,000+  │ 50,000+ │ 200,000+│100,000+│  10,000+│  5,000+ │
│ Memory Footprint        │  ~500MB   │   ~200MB   │ ~100MB  │  ~50MB  │ ~150MB │    N/A  │   N/A   │
│ Config Hot Reload       │   <1s     │   <100ms   │  <1s    │  ~1s    │ <100ms │  <30s   │  <60s   │
│ Cold Start              │   N/A     │    N/A     │   N/A   │   N/A   │  N/A   │  ~100ms │  ~500ms │
└─────────────────────────┴──────────┴────────────┴─────────┴─────────┴────────┴─────────┴─────────┘
```

### Cost Comparison (Self-Hosted vs Managed)

```
┌─────────────────────────┬──────────────────────────┬───────────────────────────────────────────┐
│        Solution         │     Self-Hosted Cost      │            Managed Cost                   │
├─────────────────────────┼──────────────────────────┼───────────────────────────────────────────┤
│ Kong Gateway            │ Compute + Support license │ Kong Cloud: ~$500+/month per node       │
│ Ambassador Edge Stack   │ Compute + AES license     │ Ambassador Cloud: ~$300+/month          │
│ Traefik                 │ Compute only (free)       │ Traefik Enterprise: Custom pricing        │
│ NGINX Plus              │ Compute + Plus license    │ N/A (self-hosted only)                    │
│ Envoy                   │ Compute only (free)       │ Tetrate / Solo.io: Custom pricing         │
│ AWS API Gateway         │ N/A                       │ $1-3.50/million requests + data transfer│
│ Azure API Management    │ N/A                       │ ~$200-3000+/month per instance            │
│ Cloudflare API Gateway  │ N/A                       │ Enterprise plan: Custom pricing           │
└─────────────────────────┴──────────────────────────┴───────────────────────────────────────────┘
```

---

## Selection Criteria

### Decision Matrix for Portalis

```
┌─────────────────────────────────────────────────────────────────┐
│              PORTALIS GATEWAY SELECTION CRITERIA                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│   REQUIREMENT                    WEIGHT    Kong   Ambassador Traefik│
│   ─────────────────────────────────────────────────────────────  │
│                                                                   │
│   Kubernetes Native               10        9        10       10  │
│   Performance (<10ms p99)         10        8         9        9   │
│   Hot Reload Capability            8        8        10        9   │
│   Extensibility (Plugins)          9        9         7        7   │
│   Developer Portal                 7        6        10        4   │
│   Rate Limiting (Advanced)         9        9         8        8   │
│   Authentication Flexibility       9        9         9        8   │
│   Observability                    8        9         9        9   │
│   Documentation Quality            7        8         9        9   │
│   Community Support                6        8         7        8   │
│   Enterprise Support               5        9         8        5   │
│   Multi-tenancy Support            8        8         7        6   │
│                                                                   │
│   ─────────────────────────────────────────────────────────────  │
│                                                                   │
│   WEIGHTED TOTAL                             823      860      810 │
│   (Max: 1060)                                                    │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### Recommendation Summary

| Scenario | Recommended Solution | Rationale |
|----------|---------------------|-----------|
| **Kubernetes-First** | Ambassador Edge Stack | Native K8s integration, Envoy performance |
| **Maximum Flexibility** | Kong Gateway | Plugin ecosystem, hybrid deployment |
| **Simple/Small Scale** | Traefik | Easy configuration, great documentation |
| **Maximum Performance** | NGINX Plus / Envoy | Lowest latency, highest throughput |
| **Cloud-Native (AWS)** | AWS API Gateway | Managed, integrated with AWS ecosystem |
| **Cloud-Native (Azure)** | Azure APIM | Strong policy engine, developer portal |
| **Edge/Global** | Cloudflare | Distributed edge, DDoS protection |

---

## References

### Official Documentation

| Gateway | Documentation URL |
|---------|-------------------|
| Kong | https://docs.konghq.com/ |
| Ambassador | https://www.getambassador.io/docs/ |
| Traefik | https://doc.traefik.io/traefik/ |
| NGINX Plus | https://docs.nginx.com/nginx/ |
| Envoy | https://www.envoyproxy.io/docs/ |
| AWS API Gateway | https://docs.aws.amazon.com/apigateway/ |
| Azure APIM | https://docs.microsoft.com/azure/api-management/ |
| Cloudflare | https://developers.cloudflare.com/api-shield/ |

### Technical Specifications

1. **Kong Gateway** - https://docs.konghq.com/gateway/
2. **Envoy xDS Protocol** - https://www.envoyproxy.io/docs/envoy/latest/api-docs/xds_protocol
3. **OpenAPI Specification** - https://spec.openapis.org/oas/latest.html
4. **Kubernetes Gateway API** - https://gateway-api.sigs.k8s.io/

### Research Papers & Articles

1. "API Gateway Patterns" - https://microservices.io/patterns/apigateway.html
2. "Envoy Proxy Architecture Overview" - https://www.envoyproxy.io/docs/envoy/latest/intro/arch_overview
3. "NGINX Performance Tuning" - https://www.nginx.com/blog/tuning-nginx/
4. "Rate Limiting Strategies" - https://medium.com/@sahilgulati007/rate-limiting-strategies-520f98bab874

---

*End of Document - API Gateways State of the Art*

**Document Stats:**
- Total Lines: ~1800
- Gateways Covered: 8
- Comparison Tables: 5
- Configuration Examples: 12+
