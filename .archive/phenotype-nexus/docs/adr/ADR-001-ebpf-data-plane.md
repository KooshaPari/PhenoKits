# ADR-001: eBPF-Based Data Plane Architecture

**Status**: Accepted  
**Date**: 2026-04-05  
**Deciders**: phenotype-nexus Architecture Team  
**Category**: Architecture / Data Plane

---

## Context

phenotype-nexus requires a high-performance data plane for service-to-service communication that can handle enterprise-scale workloads with minimal overhead. The traditional approach uses sidecar proxies (Envoy, Linkerd2-proxy) which introduce significant resource consumption and latency.

We need to select a data plane technology that balances:
- Performance (latency, throughput, resource efficiency)
- Security (mTLS, network policies)
- Operational complexity (deployment, upgrades, debugging)
- Feature completeness (L7 routing, observability)

---

## Decision

**We will use eBPF (Extended Berkeley Packet Filter) as the foundation of our data plane architecture**, specifically leveraging Cilium's eBPF implementation for L3/L4 processing with custom eBPF programs for L7 functionality where feasible.

### Rationale

| Factor | Sidecar (Envoy) | Sidecar (Linkerd2) | eBPF (Cilium) | Assessment |
|--------|-----------------|-------------------|---------------|------------|
| **Latency overhead** | 0.5-1ms | 0.2-0.5ms | 0.1-0.2ms | eBPF 2-5x better |
| **Memory per service** | 50-100MB | 20-40MB | 0MB (kernel) | eBPF eliminates overhead |
| **CPU overhead** | 5-10% | 2-5% | 1-2% | eBPF 50-80% better |
| **Deployment complexity** | High (per-pod) | Medium (per-pod) | Low (node-level) | eBPF simpler |
| **L7 capabilities** | Full | Basic | Partial | Sidecars win on features |
| **Production maturity** | 7+ years | 6+ years | 4+ years | All production-ready |
| **Debuggability** | Excellent | Good | Complex | Trade-off accepted |

### Selected Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    Kubernetes Node                               │
├─────────────────────────────────────────────────────────────────┤
│  ┌───────────────────────────────────────────────────────────┐  │
│  │              phenotype-nexus eBPF Data Plane               │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐       │  │
│  │  │ L3/L4       │  │ L7          │  │ mTLS        │       │  │
│  │  │ Forwarding  │  │ Parser      │  │ Termination │       │  │
│  │  │ (tc/ XDP)   │  │ (sockops)   │  │ (kTLS)      │       │  │
│  │  └─────────────┘  └─────────────┘  └─────────────┘       │  │
│  │                                                           │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐       │  │
│  │  │ Network     │  │ Observability│  │ Policy      │       │  │
│  │  │ Policies    │  │ (BPF maps)  │  │ Enforcement │       │  │
│  │  └─────────────┘  └─────────────┘  └─────────────┘       │  │
│  └───────────────────────────────────────────────────────────┘  │
│                              │                                  │
│  ┌─────────────────────────────▼─────────────────────────────┐  │
│  │                    Pod Network Namespace                   │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐                  │  │
│  │  │ App      │  │ App      │  │ App      │  ...           │  │
│  │  │ Container│  │ Container│  │ Container│                  │  │
│  │  │ (no      │  │ (no      │  │ (no      │                  │  │
│  │  │ sidecar) │  │ sidecar) │  │ sidecar) │                  │  │
│  │  └──────────┘  └──────────┘  └──────────┘                  │  │
│  └─────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

### Key Components

1. **L3/L4 Processing (tc/XDP)**: eBPF programs attached to traffic control hooks for packet filtering, forwarding, and load balancing at wire speed

2. **Socket Operations (sockops)**: eBPF programs for socket-level traffic management enabling L7 parsing without full proxy termination

3. **BPF Maps**: Shared data structures between kernel and userspace for:
   - Service discovery state
   - Connection tracking
   - Metrics aggregation
   - Policy configuration

4. **kTLS Integration**: Kernel TLS for hardware-accelerated mTLS termination when available

5. **Userspace Control**: Minimal userspace agent for complex L7 processing (WebSocket, gRPC-Web) that only activates when needed

---

## Consequences

### Positive

1. **10x Performance Improvement**: eBPF enables kernel-level packet processing without userspace transitions
2. **Zero Memory Overhead**: No per-pod containers required, enabling dense service packing
3. **Simplified Operations**: Node-level deployment vs. per-pod sidecar management
4. **Strong Security**: Kernel-level enforcement of network policies and mTLS
5. **Future-Proof**: eBPF is rapidly becoming the standard for cloud-native infrastructure

### Negative

1. **Debuggability Challenges**: Kernel-level execution requires specialized tooling (bpftrace, BCC)
2. **L7 Limitations**: Complex L7 protocols may require fallback to userspace proxies
3. **Kernel Requirements**: Requires Linux 5.10+ for full feature set
4. **Development Complexity**: eBPF development has steep learning curve and verification constraints
5. **Vendor Lock-in**: Cilium-specific extensions may complicate migration

### Mitigations

| Risk | Mitigation Strategy | Owner | Timeline |
|------|---------------------|-------|----------|
| Debuggability | Build integrated observability (Hubble-like) + documentation | Platform Team | 2026-Q2 |
| L7 Limitations | Hybrid approach with per-namespace waypoint proxies for complex protocols | Data Plane Team | 2026-Q2 |
| Kernel requirements | Maintain compatibility matrix; support 5.4+ with graceful degradation | Infrastructure Team | Ongoing |
| Development complexity | eBPF training program + abstraction libraries | Engineering | 2026-Q1 |
| Vendor lock-in | Abstract Cilium APIs behind phenotype-nexus interfaces | Architecture | 2026-Q2 |

---

## Alternatives Considered

### Option A: Envoy Sidecar (Istio-style)
- **Pros**: Full L7 capabilities, mature ecosystem, extensive documentation
- **Cons**: 50-100MB memory per service, 0.5-1ms latency, complex upgrades
- **Verdict**: Rejected due to resource overhead at scale

### Option B: Linkerd2 Rust Proxy
- **Pros**: Memory-safe, lower resource usage than Envoy, good performance
- **Cons**: Still per-pod overhead (20-40MB), limited L7 features
- **Verdict**: Rejected; best-of-breed sidecar but sidecars fundamentally inefficient

### Option C: Istio Ambient Mode
- **Pros**: Sidecar-less with waypoint proxies for L7, Istio ecosystem
- **Cons**: Very new (<1 year), complex ztunnel/waypoint architecture, limited production history
- **Verdict**: Rejected; promising but immature for enterprise deployment

### Option D: Cilium Service Mesh
- **Pros**: Production eBPF, strong performance, growing adoption
- **Cons**: Limited L7 compared to Envoy, Cilium-specific APIs
- **Verdict**: **Selected** with custom extensions for phenotype-nexus requirements

---

## Implementation Plan

| Phase | Deliverable | Timeline | Dependencies |
|-------|-------------|----------|--------------|
| Phase 1 | Cilium integration, basic L3/L4 | 2026-Q1 | Kernel 5.10+ |
| Phase 2 | Custom eBPF L7 parser | 2026-Q2 | Phase 1, sockops research |
| Phase 3 | mTLS via kTLS/Spire | 2026-Q2 | Phase 2, SPIFFE integration |
| Phase 4 | Waypoint proxy for complex L7 | 2026-Q3 | Phase 3, Envoy integration |
| Phase 5 | Production hardening, GA | 2026-Q4 | All prior phases |

---

## References

| Reference | URL | Description |
|-----------|-----|-------------|
| Cilium eBPF Data Path | https://docs.cilium.io/en/stable/concepts/ebpf/intro/ | Core eBPF architecture |
| eBPF Verification | https://docs.kernel.org/bpf/verifier.html | Kernel verification constraints |
| XDP Introduction | https://www.iovisor.org/technology/xdp | XDP fast path documentation |
| kTLS Documentation | https://www.kernel.org/doc/html/latest/networking/tls-offload.html | Kernel TLS offload |
| Cilium Service Mesh | https://docs.cilium.io/en/stable/network/servicemesh/ | Cilium mesh capabilities |
| sockops BPF | https://lwn.net/Articles/727829/ | Socket operations with eBPF |

---

## Related Decisions

- ADR-002: Control Plane Architecture with Raft Consensus
- ADR-003: Observability Strategy with OpenTelemetry

---

*Last updated: 2026-04-05*  
*Next review: 2026-07-05*
