# Architecture Decision Records - Schemaforge

## ADR-001: Multi-Format Schema Engine Architecture

### Status
**Accepted** - 2026-04-05

### Context

Schemaforge aims to support multiple schema definition formats (JSON Schema, Protocol Buffers, GraphQL SDL, OpenAPI, Avro) in a unified system. We needed to decide on an architecture that:

1. Allows format-specific optimizations while maintaining a common interface
2. Enables cross-format compatibility checking
3. Supports format-agnostic code generation
4. Minimizes maintenance overhead as new formats are added

We considered three architectural approaches:

| Approach | Description | Pros | Cons |
|----------|-------------|------|------|
| **A. Monolithic Parser** | Single parser with format-specific branches | Simple initially | Unmaintainable, tight coupling |
| **B. Separate Services** | Microservice per format | Independent scaling | Operational complexity, latency |
| **C. Plugin Architecture** | Common interface with pluggable parsers | Extensible, testable, decoupled | Initial design effort |

### Decision

**Adopt Approach C: Plugin Architecture with a Unified Core**

The architecture consists of:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        Schemaforge Multi-Format Engine                        │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                        Unified Core Interface                        │   │
│  │                                                                       │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐ │   │
│  │  │   Schema    │  │   Parser    │  │  Validator  │  │   CodeGen   │ │   │
│  │  │   Trait     │  │   Trait     │  │   Trait     │  │   Trait     │ │   │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘ │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                        │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                     Format-Specific Plugins                          │   │
│  │                                                                       │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐ │   │
│  │  │ JSON Schema │  │  Protobuf   │  │   GraphQL   │  │   OpenAPI   │ │   │
│  │  │   Plugin    │  │   Plugin    │  │   Plugin    │  │   Plugin    │ │   │
│  │  │  (Rust)     │  │  (Rust)     │  │  (Rust)     │  │  (Rust)     │ │   │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘ │   │
│  │                                                                       │   │
│  │  ┌─────────────┐  ┌─────────────┐                                    │   │
│  │  │    Avro     │  │   Custom    │  ← Future formats...               │   │
│  │  │   Plugin    │  │   Plugin    │                                    │   │
│  │  └─────────────┘  └─────────────┘                                    │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Core Traits

```rust
// Unified Schema representation
pub trait Schema: Send + Sync {
    fn id(&self) -> &SchemaId;
    fn format(&self) -> SchemaFormat;
    fn canonical(&self) -> &CanonicalSchema;
    fn source(&self) -> &str;
    fn metadata(&self) -> &SchemaMetadata;
}

// Parser plugin interface
#[async_trait]
pub trait SchemaParser: Send + Sync {
    fn format(&self) -> SchemaFormat;
    async fn parse(&self, input: &str) -> Result<Box<dyn Schema>, ParseError>;
    async fn canonicalize(&self, schema: &dyn Schema) -> Result<CanonicalSchema, ParseError>;
}

// Validator plugin interface
#[async_trait]
pub trait SchemaValidator: Send + Sync {
    fn supported_formats(&self) -> Vec<SchemaFormat>;
    async fn validate_schema(&self, schema: &dyn Schema) -> Result<ValidationReport, ValidationError>;
    async fn validate_data(&self, schema: &dyn Schema, data: &Value) -> Result<ValidationReport, ValidationError>;
    async fn check_compatibility(&self, old: &dyn Schema, new: &dyn Schema) -> Result<CompatibilityReport, CompatibilityError>;
}

// Code generator interface
#[async_trait]
pub trait CodeGenerator: Send + Sync {
    fn supported_formats(&self) -> Vec<SchemaFormat>;
    fn target_languages(&self) -> Vec<Language>;
    async fn generate(&self, schema: &dyn Schema, options: &GenOptions) -> Result<GeneratedCode, GenerationError>;
}
```

### Consequences

**Positive:**
- ✅ New formats can be added without modifying core code
- ✅ Each format can use optimal parsing libraries (e.g., `jsonschema` crate for JSON Schema, `prost` for Protobuf)
- ✅ Format-specific optimizations are isolated and testable
- ✅ Cross-format conversion is possible via canonical intermediate representation
- ✅ Third-party plugins are feasible for custom formats

**Negative:**
- ⚠️ Initial implementation requires careful trait design
- ⚠️ Some overhead from trait object dispatch (mitigated by caching)
- ⚠️ Plugin registration and discovery adds complexity

### Implementation

```rust
// Plugin registration
pub struct SchemaEngine {
    parsers: HashMap<SchemaFormat, Box<dyn SchemaParser>>,
    validators: HashMap<SchemaFormat, Box<dyn SchemaValidator>>,
    generators: Vec<Box<dyn CodeGenerator>>,
}

impl SchemaEngine {
    pub fn register_parser(&mut self, parser: Box<dyn SchemaParser>) {
        self.parsers.insert(parser.format(), parser);
    }

    pub async fn parse(&self, format: SchemaFormat, input: &str) -> Result<Box<dyn Schema>, EngineError> {
        let parser = self.parsers.get(&format)
            .ok_or(EngineError::UnsupportedFormat(format))?;
        parser.parse(input).await.map_err(EngineError::from)
    }
}

// JSON Schema plugin implementation
pub struct JsonSchemaPlugin;

#[async_trait]
impl SchemaParser for JsonSchemaPlugin {
    fn format(&self) -> SchemaFormat { SchemaFormat::JsonSchema }

    async fn parse(&self, input: &str) -> Result<Box<dyn Schema>, ParseError> {
        let compiled = jsonschema::JSONSchema::compile(&serde_json::from_str(input)?)?;
        Ok(Box::new(JsonSchemaImpl { compiled, source: input.to_string() }))
    }
}
```

### Related Decisions

- ADR-002: Storage Architecture for Multi-Format Schemas
- ADR-003: Validation Engine Design

### References

- [Rust Plugin Architecture Patterns](https://github.com/rust-unofficial/patterns/blob/main/patterns/abstract-factory.md)
- [jsonschema Rust Crate](https://docs.rs/jsonschema/latest/jsonschema/)
- [prost Protobuf Implementation](https://docs.rs/prost/latest/prost/)

---

## ADR-002: Storage Architecture - PostgreSQL with JSONB + Content-Addressed Cache

### Status
**Accepted** - 2026-04-05

### Context

Schemaforge requires a storage system that supports:

1. Schema versioning with immutable history
2. Fast lookup by ID, name, version, and content hash
3. Complex queries on schema metadata
4. Full-text search across schema content
5. Efficient storage of large schemas (up to 5MB)
6. ACID compliance for schema publication
7. Horizontal scalability for high-availability deployments

We evaluated three storage approaches:

| Approach | Description | Pros | Cons |
|----------|-------------|------|------|
| **A. Pure PostgreSQL** | Relational storage with JSONB | Full ACID, rich queries, proven | Single-node limitations |
| **B. Document Store (MongoDB)** | Schema-per-document | Flexible, horizontal scale | Less consistent, complex transactions |
| **C. Hybrid (PG + Redis)** | PG for persistence, Redis for cache | Speed + reliability, query cache | Operational complexity |

### Decision

**Adopt Approach C: PostgreSQL with JSONB as Source of Truth, Redis for Hot Cache and Content-Addressed Deduplication**

### Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         Schemaforge Storage Layer                            │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         Redis Cache Layer                            │   │
│  │                                                                       │   │
│  │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐      │   │
│  │  │  Schema Lookup  │  │  Content Hash   │  │  Query Results  │      │   │
│  │  │    (LRU)        │  │    Index        │  │    Cache        │      │   │
│  │  │                 │  │  (Bloom Filter) │  │                 │      │   │
│  │  │  TTL: 1 hour    │  │                 │  │  TTL: 5 min     │      │   │
│  │  └─────────────────┘  └─────────────────┘  └─────────────────┘      │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                        │
│                         Miss / Write-back                                  │
│                                    │                                        │
│                                    ▼                                        │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    PostgreSQL Primary Store                          │   │
│  │                                                                       │   │
│  │  ┌─────────────────────────────────────────────────────────────────┐ │   │
│  │  │                    schemas Table                                 │ │   │
│  │  │  id UUID PRIMARY KEY                                           │ │   │
│  │  │  name VARCHAR(255) NOT NULL                                    │ │   │
│  │  │  version_major/minor/patch INT                                 │ │   │
│  │  │  format VARCHAR(50) NOT NULL                                   │ │   │
│  │  │  content_hash VARCHAR(64) UNIQUE                               │ │   │
│  │  │  content TEXT NOT NULL                                         │ │   │
│  │  │  metadata JSONB                                                │ │   │
│  │  │  search_vector TSVECTOR                                        │ │   │
│  │  │  created_at TIMESTAMPTZ                                        │ │   │
│  │  │  deprecated BOOLEAN DEFAULT FALSE                              │ │   │
│  │  └─────────────────────────────────────────────────────────────────┘ │   │
│  │                                                                       │   │
│  │  ┌─────────────────────────────────────────────────────────────────┐ │   │
│  │  │                    schema_dependencies Table                     │ │   │
│  │  │  schema_id UUID REFERENCES schemas(id)                           │ │   │
│  │  │  dependency_id UUID REFERENCES schemas(id)                     │ │   │
│  │  │  dependency_type VARCHAR(50) -- $ref, import, etc.               │ │   │
│  │  └─────────────────────────────────────────────────────────────────┘ │   │
│  │                                                                       │   │
│  │  ┌─────────────────────────────────────────────────────────────────┐ │   │
│  │  │                    schema_versions View                        │ │   │
│  │  │  -- Materialized view for fast version listing                 │ │   │
│  │  └─────────────────────────────────────────────────────────────────┘ │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    Optional: S3/GCS for Large Schemas                │   │
│  │  -- Schemas >100KB stored externally, referenced by URL            │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Schema Design

```sql
-- Core schemas table with JSONB for flexibility
CREATE TABLE schemas (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    namespace VARCHAR(255) DEFAULT 'default',
    version_major INT NOT NULL,
    version_minor INT NOT NULL,
    version_patch INT NOT NULL,
    format VARCHAR(50) NOT NULL,
    
    -- Content addressing for deduplication
    content_hash VARCHAR(64) NOT NULL,
    content TEXT NOT NULL,
    
    -- Compressed storage for large schemas
    content_compressed BYTEA,
    compression_algorithm VARCHAR(20),
    
    -- Metadata as JSONB for extensibility
    metadata JSONB DEFAULT '{}',
    
    -- Full-text search support
    search_vector TSVECTOR,
    
    -- Audit fields
    author VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deprecated BOOLEAN NOT NULL DEFAULT FALSE,
    deprecation_reason TEXT,
    superseded_by UUID REFERENCES schemas(id),
    
    -- Constraints
    CONSTRAINT unique_version UNIQUE (namespace, name, version_major, version_minor, version_patch),
    CONSTRAINT unique_content UNIQUE (content_hash)
);

-- Indexes for common queries
CREATE INDEX idx_schemas_name ON schemas(namespace, name);
CREATE INDEX idx_schemas_format ON schemas(format);
CREATE INDEX idx_schemas_created ON schemas(created_at DESC);
CREATE INDEX idx_schemas_metadata ON schemas USING GIN(metadata jsonb_path_ops);
CREATE INDEX idx_schemas_search ON schemas USING GIN(search_vector);

-- Dependency tracking for schema references
CREATE TABLE schema_dependencies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    schema_id UUID NOT NULL REFERENCES schemas(id) ON DELETE CASCADE,
    dependency_ref VARCHAR(255) NOT NULL, -- Reference string ($ref, import, etc.)
    dependency_schema_id UUID REFERENCES schemas(id), -- Resolved reference
    dependency_type VARCHAR(50) NOT NULL, -- 'internal', 'external', 'system'
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_deps_schema ON schema_dependencies(schema_id);
CREATE INDEX idx_deps_dependency ON schema_dependencies(dependency_schema_id);

-- Compatibility check results caching
CREATE TABLE compatibility_checks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    old_schema_id UUID NOT NULL REFERENCES schemas(id),
    new_schema_id UUID NOT NULL REFERENCES schemas(id),
    is_compatible BOOLEAN NOT NULL,
    compatibility_level VARCHAR(50), -- 'none', 'backward', 'forward', 'full'
    breaking_changes JSONB,
    warnings JSONB,
    checked_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    checked_by VARCHAR(255),
    
    CONSTRAINT unique_check UNIQUE (old_schema_id, new_schema_id)
);

-- Trigger for full-text search maintenance
CREATE OR REPLACE FUNCTION update_search_vector()
RETURNS TRIGGER AS $$
BEGIN
    NEW.search_vector := 
        setweight(to_tsvector('english', COALESCE(NEW.name, '')), 'A') ||
        setweight(to_tsvector('english', COALESCE(NEW.metadata->>'description', '')), 'B') ||
        setweight(to_tsvector('english', COALESCE(NEW.content, '')), 'C');
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER schemas_search_update
    BEFORE INSERT OR UPDATE ON schemas
    FOR EACH ROW
    EXECUTE FUNCTION update_search_vector();
```

### Caching Strategy

| Cache Type | Key Pattern | TTL | Purpose |
|------------|-------------|-----|---------|
| **Schema by ID** | `schema:id:{uuid}` | 1 hour | Direct lookups |
| **Schema by Version** | `schema:{ns}:{name}:{ver}` | 1 hour | Version lookups |
| **Content Hash** | `schema:hash:{sha256}` | 24 hours | Deduplication |
| **Search Results** | `search:{query_hash}` | 5 min | Query caching |
| **Compatibility** | `compat:{old_id}:{new_id}` | 1 hour | Validation results |
| **Dependency Graph** | `deps:{schema_id}` | 30 min | Dependency resolution |

```rust
pub struct SchemaCache {
    redis: redis::aio::MultiplexedConnection,
    bloom_filter: BloomFilter, // For hash-based lookups
}

impl SchemaCache {
    pub async fn get_by_hash(&self, hash: &str) -> Option<SchemaSummary> {
        // Check bloom filter first (no false negatives)
        if !self.bloom_filter.contains(hash) {
            return None;
        }
        // Check Redis
        let key = format!("schema:hash:{}", hash);
        self.redis.get(&key).await.ok().flatten()
    }

    pub async fn warm_cache(&self, schema: &Schema) {
        let id_key = format!("schema:id:{}", schema.id);
        let hash_key = format!("schema:hash:{}", schema.content_hash);
        let ver_key = format!("schema:{}:{}:{}.{}.{}",
            schema.namespace, schema.name,
            schema.version.major, schema.version.minor, schema.version.patch);

        let pipe = redis::pipe()
            .set(&id_key, schema).expire(&id_key, 3600)
            .set(&hash_key, schema.summary()).expire(&hash_key, 86400)
            .set(&ver_key, schema).expire(&ver_key, 3600)
            .query_async(&mut self.redis).await;
    }
}
```

### Consequences

**Positive:**
- ✅ ACID transactions ensure schema publication integrity
- ✅ JSONB provides flexibility for format-specific metadata
- ✅ Full-text search enables discovery
- ✅ Content addressing prevents duplicate storage
- ✅ Redis cache reduces database load for hot schemas
- ✅ PostgreSQL's reliability and ecosystem

**Negative:**
- ⚠️ Redis adds operational complexity (clustering, failover)
- ⚠️ Large schemas require external storage (S3)
- ⚠️ Write amplification from cache invalidation

### Migration Path

1. **Phase 1**: PostgreSQL primary store, no cache
2. **Phase 2**: Add Redis for hot schema caching
3. **Phase 3**: Add content-addressed deduplication
4. **Phase 4**: Add S3 offload for large schemas

### References

- [PostgreSQL JSONB Performance](https://www.postgresql.org/docs/current/datatype-json.html)
- [Content-Addressed Storage](https://docs.ipfs.tech/concepts/content-addressing/)
- [Redis Caching Patterns](https://redis.io/docs/manual/client-side-caching/)

---

## ADR-003: Validation Engine - Streaming Async with Partial Results

### Status
**Accepted** - 2026-04-05

### Context

The validation engine must handle:

1. Large schemas (up to 5MB) with deep nesting
2. High-throughput validation (1000+ validations/second)
3. Multiple validation types: syntax, semantic, compatibility
4. Streaming data validation for large payloads
5. Partial results for progressive error reporting
6. Resource limits: CPU, memory, validation depth

We evaluated three approaches:

| Approach | Description | Pros | Cons |
|----------|-------------|------|------|
| **A. Synchronous Batch** | Validate entire schema at once | Simple | Blocking, no progress |
| **B. Async with Full Results** | Async but return all at end | Non-blocking | High memory, slow feedback |
| **C. Streaming Async with Partial** | Stream results as found | Memory efficient, fast feedback | Complex implementation |

### Decision

**Adopt Approach C: Streaming Async Validation with Partial Results and Resource Limits**

### Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     Schemaforge Validation Engine                           │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                     Validation Pipeline                              │   │
│  │                                                                       │   │
│  │   Input → Parse → Structural → Semantic → Compatibility → Output     │   │
│  │              ↓         ↓           ↓            ↓                    │   │
│  │         SyntaxErr  StructureErr  LogicErr   BreakingErr            │   │
│  │              └─────────┴───────────┴───────────┘                     │   │
│  │                          │                                          │   │
│  │                          ▼                                          │   │
│  │              ┌─────────────────────┐                                │   │
│  │              │   Stream Results    │                                │   │
│  │              │   (mpsc channel)    │                                │   │
│  │              └─────────────────────┘                                │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                     Resource Guardian                                │   │
│  │                                                                       │   │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐  ┌────────────┐      │   │
│  │  │  Timeout   │  │  Memory    │  │   Depth    │  │   Rate     │      │   │
│  │  │   30s      │  │   256MB    │  │   50 lvl   │  │  1000/s    │      │   │
│  │  └────────────┘  └────────────┘  └────────────┘  └────────────┘      │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Implementation

```rust
// Validation stages as an enum for progress tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum ValidationStage {
    Parsing,
    Structural,
    Semantic,
    Compatibility,
    Complete,
}

// Partial validation result streaming
#[derive(Debug, Clone, Serialize)]
pub enum ValidationEvent {
    StageStarted { stage: ValidationStage },
    StageCompleted { stage: ValidationStage, duration_ms: u64 },
    IssueFound { 
        stage: ValidationStage,
        severity: Severity,
        path: String,
        message: String,
        code: String,
    },
    Progress { stage: ValidationStage, percent: u8 },
    Complete(ValidationReport),
    Error(ValidationError),
}

// Main validation engine
pub struct ValidationEngine {
    stages: Vec<Box<dyn ValidationStageExecutor>>,
    resource_guard: ResourceGuard,
    event_tx: mpsc::Sender<ValidationEvent>,
}

#[async_trait]
pub trait ValidationStageExecutor: Send + Sync {
    fn stage(&self) -> ValidationStage;
    async fn execute(
        &self,
        input: &ValidationInput,
        context: &ValidationContext,
        event_tx: mpsc::Sender<ValidationEvent>,
    ) -> Result<StageOutput, StageError>;
}

impl ValidationEngine {
    pub async fn validate_streaming(
        &self,
        input: ValidationInput,
    ) -> mpsc::Receiver<ValidationEvent> {
        let (tx, rx) = mpsc::channel(100);
        let engine = self.clone();

        tokio::spawn(async move {
            let _guard = engine.resource_guard.acquire().await;
            let ctx = ValidationContext::new(&input);

            for stage in &engine.stages {
                let stage_tx = tx.clone();
                let _ = tx.send(ValidationEvent::StageStarted { 
                    stage: stage.stage() 
                }).await;

                let start = Instant::now();
                
                match stage.execute(&input, &ctx, stage_tx.clone()).await {
                    Ok(output) => {
                        let duration = start.elapsed().as_millis() as u64;
                        let _ = stage_tx.send(ValidationEvent::StageCompleted { 
                            stage: stage.stage(), 
                            duration_ms: duration 
                        }).await;
                        ctx.update(output);
                    }
                    Err(e) => {
                        let _ = stage_tx.send(ValidationEvent::Error(e.into())).await;
                        return;
                    }
                }
            }

            let report = ctx.into_report();
            let _ = tx.send(ValidationEvent::Complete(report)).await;
        });

        rx
    }
}

// Resource limits enforcement
pub struct ResourceGuard {
    timeout: Duration,
    max_memory: usize,
    max_depth: usize,
    semaphore: Semaphore,
}

impl ResourceGuard {
    pub async fn acquire(&self) -> ResourceGuardHandle {
        let _permit = self.semaphore.acquire().await.unwrap();
        ResourceGuardHandle {
            deadline: Instant::now() + self.timeout,
            max_memory: self.max_memory,
            max_depth: self.max_depth,
            _permit,
        }
    }
}

// Depth tracking for nested schemas
pub struct DepthTracker {
    current: AtomicUsize,
    max: usize,
}

impl DepthTracker {
    pub fn enter(&self) -> Result<DepthGuard, ValidationError> {
        let current = self.current.fetch_add(1, Ordering::SeqCst);
        if current > self.max {
            return Err(ValidationError::MaxDepthExceeded {
                max: self.max,
                path: String::new(),
            });
        }
        Ok(DepthGuard { tracker: self })
    }
}
```

### JSON Schema Specific Implementation

```rust
pub struct JsonSchemaValidationStage {
    compiler: Arc<JSONSchema>,
}

#[async_trait]
impl ValidationStageExecutor for JsonSchemaValidationStage {
    fn stage(&self) -> ValidationStage { ValidationStage::Semantic }

    async fn execute(
        &self,
        input: &ValidationInput,
        _ctx: &ValidationContext,
        event_tx: mpsc::Sender<ValidationEvent>,
    ) -> Result<StageOutput, StageError> {
        let schema = self.compiler.compile(&input.schema_json)?;
        let mut issues = Vec::new();

        // Validate each keyword with progress updates
        for (idx, (keyword, value)) in input.schema_json.as_object().unwrap().iter().enumerate() {
            if let Err(e) = validate_keyword(&schema, keyword, value) {
                let issue = ValidationEvent::IssueFound {
                    stage: self.stage(),
                    severity: Severity::Error,
                    path: format!("#/{}", keyword),
                    message: e.to_string(),
                    code: "INVALID_KEYWORD".to_string(),
                };
                let _ = event_tx.send(issue).await;
                issues.push(e);
            }

            // Send progress every 10 keywords
            if idx % 10 == 0 {
                let percent = (idx * 100) / input.schema_json.as_object().unwrap().len();
                let _ = event_tx.send(ValidationEvent::Progress { 
                    stage: self.stage(), 
                    percent: percent as u8 
                }).await;
            }
        }

        Ok(StageOutput { issues, warnings: vec![] })
    }
}
```

### Performance Characteristics

| Metric | Target | Implementation |
|--------|--------|----------------|
| Small schema (<10KB) | <10ms | Compiled once, cached |
| Medium schema (<100KB) | <50ms | Streaming validation |
| Large schema (<1MB) | <200ms | Incremental with checkpoints |
| Very large (>1MB) | <500ms | Parallel validation, chunked |
| Concurrent validations | 1000/s | Semaphore + backpressure |
| Memory per validation | <10MB | Streaming, no full AST |
| Cancellation latency | <10ms | Cooperative yield points |

### Consequences

**Positive:**
- ✅ Non-blocking validation allows high concurrency
- ✅ Streaming results enable real-time UI updates
- ✅ Resource limits prevent DoS from malicious schemas
- ✅ Partial results allow early termination on fatal errors
- ✅ Checkpoint/resume for very large schemas

**Negative:**
- ⚠️ Complex error aggregation across stages
- ⚠️ Async overhead for very small schemas
- ⚠️ Requires careful cancellation handling

### Testing Strategy

```rust
#[tokio::test]
async fn test_streaming_validation() {
    let engine = ValidationEngine::new();
    let input = test_schema_input();
    
    let mut rx = engine.validate_streaming(input).await;
    let mut events = vec![];
    
    while let Some(event) = rx.recv().await {
        events.push(event);
    }
    
    // Verify stage progression
    assert!(events.iter().any(|e| matches!(e, 
        ValidationEvent::StageStarted { stage: ValidationStage::Parsing })));
    assert!(events.iter().any(|e| matches!(e, 
        ValidationEvent::StageStarted { stage: ValidationStage::Structural })));
    
    // Verify completion
    assert!(matches!(events.last(), Some(ValidationEvent::Complete(_))));
}

#[tokio::test]
async fn test_resource_limits() {
    let engine = ValidationEngine::builder()
        .max_depth(5)
        .timeout(Duration::from_millis(100))
        .build();
    
    let deeply_nested = deeply_nested_schema(10); // Exceeds limit
    let mut rx = engine.validate_streaming(deeply_nested).await;
    
    let result = rx.recv().await;
    assert!(matches!(result, Some(ValidationEvent::Error(
        ValidationError::MaxDepthExceeded { .. }
    ))));
}
```

### References

- [Async Rust Book - Streams](https://rust-lang.github.io/async-book/05_streams/01_chapter.html)
- [Backpressure Patterns](https://medium.com/@jayphelps/backpressure-explained-the-flow-of-data-through-software-2350b3e27780)
- [JSON Schema Validation Performance](https://json-schema.org/blog/posts/benchmark-update)

---

## ADR Index

| ADR | Title | Status | Date |
|-----|-------|--------|------|
| 001 | Multi-Format Schema Engine Architecture | Accepted | 2026-04-05 |
| 002 | Storage Architecture - PostgreSQL + Redis | Accepted | 2026-04-05 |
| 003 | Validation Engine - Streaming Async | Accepted | 2026-04-05 |

---

*Last Updated: 2026-04-05*
