# ADR-002: Control Plane Architecture with Raft Consensus

**Status**: Accepted  
**Date**: 2026-04-05  
**Deciders**: phenotype-nexus Architecture Team  
**Category**: Architecture / Control Plane

---

## Context

phenotype-nexus requires a highly available, strongly consistent control plane to manage:
- Service discovery state
- Traffic routing policies
- Security policies (mTLS, network policies)
- Certificate lifecycle
- Observability configuration

The control plane must:
- Maintain consistency across distributed nodes
- Recover quickly from failures
- Scale horizontally with cluster size
- Provide linearizable reads for critical operations
- Support multi-region deployments

---

## Decision

**We will use the Raft consensus algorithm as the foundation of our control plane state management**, implemented via etcd as the backing store with a custom phenotype-nexus control plane layer for domain-specific logic.

### Rationale

| Algorithm | Type | Throughput | Latency | Fault Tolerance | Complexity | Our Assessment |
|-----------|------|------------|---------|-----------------|------------|----------------|
| **Raft** | CFT | 50K ops/s | 10ms | n/2 failures | Low | **Selected** |
| Paxos | CFT | 50K ops/s | 15ms | n/2 failures | High | Too complex |
| Zab | CFT | 100K ops/s | 5ms | n/2 failures | Medium | ZK-specific |
| PBFT | BFT | 5K ops/s | 100ms | n/3 failures | Very High | Overkill for internal trust |
| Tendermint | BFT | 10K ops/s | 1s | n/3 failures | High | Blockchain-focused |
| CRDTs | Conflict-free | 100K+ ops/s | <1ms | Always available | Medium | Eventual consistency |

Raft was selected because:
1. **Understandability**: Single paper, clear leader/follower model
2. **Production maturity**: etcd, Consul, CockroachDB all battle-tested
3. **Strong consistency**: Required for security policies and routing
4. **Reasonable performance**: 50K ops/s sufficient for our use cases
5. **Ecosystem tooling**: Rich monitoring, backup, and operational tools

### Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         phenotype-nexus Control Plane                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌───────────────────────────────────────────────────────────────────────┐ │
│  │                    phenotype-nexus API Layer                          │ │
│  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐       │ │
│  │  │ Policy      │ │ Service     │ │ Certificate │ │ xDS         │       │ │
│  │  │ API         │ │ Discovery   │ │ API         │ │ Server      │       │ │
│  │  │ (gRPC)      │ │ API         │ │ (gRPC)      │ │ (gRPC)      │       │ │
│  │  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘       │ │
│  └───────────────────────────────────────────────────────────────────────┘ │
│                                    │                                        │
│  ┌─────────────────────────────────▼───────────────────────────────────────┐ │
│  │                    Domain Logic Layer                                   │ │
│  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐       │ │
│  │  │ Policy      │ │ Service     │ │ Certificate │ │ xDS         │       │ │
│  │  │ Controller  │ │ Registry    │ │ Manager     │ │ Cache       │       │ │
│  │  │             │ │             │ │             │ │             │       │ │
│  │  │ - Validate  │ │ - Watch K8s │ │ - SPIFFE    │ │ - Delta     │       │ │
│  │  │   policies  │ │   services  │ │   SVIDs     │ │   updates   │       │ │
│  │  │ - Compile   │ │ - Health    │ │ - Rotation  │ │ - Version   │       │ │
│  │  │   BPF       │ │   checks    │ │ - Revocation│ │   control   │       │ │
│  │  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘       │ │
│  └───────────────────────────────────────────────────────────────────────┘ │
│                                    │                                        │
│  ┌─────────────────────────────────▼───────────────────────────────────────┐ │
│  │                    Consensus Layer (etcd/Raft)                          │ │
│  │                                                                         │ │
│  │    ┌──────────┐         ┌──────────┐         ┌──────────┐              │ │
│  │    │  Node 1  │◄───────►│  Node 2  │◄───────►│  Node 3  │              │ │
│  │    │ (Leader) │   Raft  │(Follower)│   Raft  │(Follower)│              │ │
│  │    └──────────┘         └──────────┘         └──────────┘              │ │
│  │         │                                              │               │ │
│  │         └──────────────────┬─────────────────────────┘               │ │
│  │                            │                                         │ │
│  │                   ┌─────────▼─────────┐                              │ │
│  │                   │   etcd Cluster    │                              │ │
│  │                   │   (3 or 5 nodes)│                              │ │
│  │                   └─────────────────┘                              │ │
│  │                                                                         │ │
│  └───────────────────────────────────────────────────────────────────────┘ │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Key Components

1. **API Layer**: gRPC APIs for external clients (Kubernetes operators, CLI, UI)
   - Policy API: CRUD for traffic and security policies
   - Service Discovery API: Service registration, health, metadata
   - Certificate API: mTLS certificate lifecycle management
   - xDS Server: Envoy xDS protocol for data plane configuration

2. **Domain Logic Layer**: Business logic specific to phenotype-nexus
   - Policy Controller: Validates, compiles policies to eBPF bytecode
   - Service Registry: Watches Kubernetes services, maintains health state
   - Certificate Manager: SPIFFE SVID issuance, rotation, revocation
   - xDS Cache: Efficient configuration distribution with delta updates

3. **Consensus Layer**: etcd cluster for distributed state
   - Leader election for control plane components
   - Distributed locking for certificate operations
   - Persistent storage for policies and service state
   - Watch mechanism for real-time updates

### Consistency Model

| Operation | Consistency Level | Implementation |
|-----------|-------------------|----------------|
| Policy reads | Linearizable | Read from leader or quorum |
| Policy writes | Linearizable | Raft consensus before commit |
| Service discovery | Eventual consistency | Watch-based propagation |
| Certificate issuance | Strict serializability | Distributed locks + Raft |
| xDS config | Monotonic reads | Versioned snapshots |
| Health checks | Best-effort | Async updates, timeout-based |

---

## Consequences

### Positive

1. **Strong Consistency**: Security policies and routing decisions are always consistent
2. **High Availability**: Survives up to (n-1)/2 node failures
3. **Proven Technology**: etcd is the Kubernetes backing store, battle-tested at scale
4. **Operational Simplicity**: Well-understood failure modes and recovery procedures
5. **Rich Tooling**: etcdctl, backup/restore, monitoring integrations
6. **Horizontal Scalability**: Add nodes for read scaling (with watches)

### Negative

1. **Write Latency**: ~10ms additional latency for consensus (acceptable)
2. **Split-Brain Risk**: Requires odd number of nodes, careful network partition handling
3. **Leader Bottleneck**: All writes go through leader (mitigated by batching)
4. **Storage Overhead**: Raft log requires compaction and snapshot management
5. **Operational Complexity**: Multi-node etcd cluster requires expertise

### Mitigations

| Risk | Mitigation | Implementation |
|------|------------|----------------|
| Leader bottleneck | Request batching, pipelining | etcd --max-request-bytes |
| Storage growth | Automatic compaction, defrag | etcd --auto-compaction-mode=periodic |
| Network partitions | Strict majority enforcement | etcd --heartbeat-interval, --election-timeout |
| Data loss | Continuous backup, snapshots | etcd-backup-operator |
| Recovery time | Automated failover detection | Health checks, leader election |

---

## Alternatives Considered

### Option A: Custom Raft Implementation
- **Pros**: Tailored to our exact needs, potentially higher performance
- **Cons**: Years of development, debugging distributed systems is hard
- **Verdict**: Rejected; etcd is already optimized and well-tested

### Option B: ZooKeeper with Zab
- **Pros**: Very high throughput (100K+ ops/s), Java ecosystem
- **Cons**: Complex operational model, declining ecosystem vs. etcd
- **Verdict**: Rejected; etcd is the Kubernetes standard

### Option C: CRDTs (Conflict-free Replicated Data Types)
- **Pros**: Always available, no consensus latency, high throughput
- **Cons**: Eventual consistency unacceptable for security policies
- **Verdict**: Rejected; security policies require strong consistency

### Option D: Spanner-like TrueTime
- **Pros**: External consistency, global distribution
- **Cons**: Requires specialized hardware (atomic clocks), Google proprietary
- **Verdict**: Rejected; impractical for on-premise deployments

---

## Implementation Details

### etcd Cluster Topology

| Deployment Size | etcd Nodes | Failure Tolerance | Use Case |
|-----------------|------------|-------------------|----------|
| Development | 1 (embedded) | None | Local testing |
| Small Production | 3 | 1 node | Single region |
| Medium Production | 5 | 2 nodes | Multi-AZ |
| Large Production | 5+ | 2+ nodes | Multi-region |

### Performance Targets

| Metric | Target | Measurement Method |
|--------|--------|-------------------|
| Write latency (P99) | <20ms | etcd metrics |
| Read latency (P99) | <5ms | etcd metrics |
| Throughput | 10K writes/s | Load testing |
| Leader failover | <5s | Failure injection |
| Recovery time | <30s | Automated tests |

### Monitoring

| Metric | Alert Threshold | Severity |
|--------|-----------------|----------|
| etcd leader changes | >3 in 10 min | Warning |
| etcd proposal failures | >1% | Critical |
| etcd disk fsync | >100ms | Warning |
| etcd network round-trip | >200ms | Warning |
| etcd members unhealthy | >0 | Critical |

---

## Multi-Region Considerations

For multi-region deployments, we will use **etcd federation with regional clusters**:

```
┌─────────────────────────┐     ┌─────────────────────────┐
│    Region A (Primary)   │     │    Region B (Secondary) │
│  ┌───────────────────┐  │     │  ┌───────────────────┐  │
│  │  etcd Cluster A   │  │     │  │  etcd Cluster B   │  │
│  │  (Leader)         │  │◄────┼──►  │  (Follower)       │  │
│  └───────────────────┘  │  Raft │  └───────────────────┘  │
│         │               │  (async)        │               │
│         ▼               │     │         ▼               │
│  ┌───────────────────┐  │     │  ┌───────────────────┐  │
│  │  phenotype-nexus  │  │     │  │  phenotype-nexus  │  │
│  │  Control Plane A  │  │     │  │  Control Plane B  │  │
│  │  (Primary)        │  │     │  │  (Read Replica)   │  │
│  └───────────────────┘  │     │  └───────────────────┘  │
└─────────────────────────┘     └─────────────────────────┘
```

- **Primary region**: Handles all writes, replicated to secondary
- **Secondary region**: Read-only for local latency, async replication
- **Failover**: Manual promotion of secondary (automated in future)

---

## References

| Reference | URL | Description |
|-----------|-----|-------------|
| Raft Paper | https://raft.github.io/raft.pdf | Original consensus algorithm |
| etcd Documentation | https://etcd.io/docs/v3.5/ | etcd operations guide |
| etcd Raft Library | https://github.com/etcd-io/raft | Go Raft implementation |
| Kubernetes etcd | https://kubernetes.io/docs/tasks/administer-cluster/configure-upgrade-etcd/ | K8s integration patterns |
| Linearizability | https://cs.brown.edu/~mph/HerlihyW90/p463-herlihy.pdf | Consistency definitions |
| Paxos Made Simple | https://lamport.azurewebsites.net/pubs/paxos-simple.pdf | Paxos for comparison |

---

## Related Decisions

- ADR-001: eBPF-Based Data Plane Architecture
- ADR-003: Observability Strategy with OpenTelemetry

---

*Last updated: 2026-04-05*  
*Next review: 2026-07-05*
