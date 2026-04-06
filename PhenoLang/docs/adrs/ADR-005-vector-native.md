# ADR-005: Vector-Native Architecture with pgvector

## Status
**ACCEPTED**

## Context

PhenoLang's pheno-vector module manages semantic search and AI-augmented features across the infrastructure platform. Current limitations:

- **No Semantic Search**: Documentation and logs require exact keyword matching
- **AI Integration Gap**: Cannot leverage embeddings for intelligent features
- **Documentation Discovery**: Users cannot find relevant docs by meaning
- **Log Analysis**: Error patterns not discoverable through semantic similarity
- **Code Navigation**: No intelligent code suggestions based on intent

With 681 files across 33 modules and extensive documentation, we need vector-native capabilities for:
- Semantic documentation search
- Intelligent error suggestion
- Code example retrieval
- Configuration assistance

## Decision

We will implement a **vector-native architecture** using **pgvector** for Postgres-based vector storage with hybrid (vector + text) search.

### Decision Details

#### pgvector Foundation

pgvector extends Postgres with vector data types and similarity search:

```python
# pgvector schema and indexing
VECTOR_SCHEMA = """
-- Enable pgvector extension
CREATE EXTENSION IF NOT EXISTS vector;

-- Embeddings table with metadata
CREATE TABLE IF NOT EXISTS pheno_embeddings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Content
    content TEXT NOT NULL,
    content_type VARCHAR(50) NOT NULL,  -- 'doc', 'code', 'log', 'error'
    
    -- Source tracking
    source_type VARCHAR(50),  -- 'markdown', 'python', 'config'
    source_path TEXT,
    source_id VARCHAR(255),
    
    -- Vector embedding (OpenAI: 1536d, local: 384d-768d)
    embedding vector(1536),
    
    -- Metadata for filtering
    metadata JSONB DEFAULT '{}',
    tags TEXT[],
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Partitioning support
    project_id VARCHAR(100)
);

-- HNSW index for fast approximate nearest neighbor search
CREATE INDEX IF NOT EXISTS idx_pheno_embeddings_hnsw 
ON pheno_embeddings USING hnsw (embedding vector_cosine_ops)
WITH (
    m = 16,              -- Max edges per node
    ef_construction = 64  -- Candidate pool size during build
);

-- IVFFlat index for exact search fallback
CREATE INDEX IF NOT EXISTS idx_pheno_embeddings_ivf
ON pheno_embeddings USING ivfflat (embedding vector_cosine_ops)
WITH (lists = 100);

-- GIN index for metadata filtering
CREATE INDEX IF NOT EXISTS idx_pheno_embeddings_metadata 
ON pheno_embeddings USING GIN (metadata);

-- Full-text search index
CREATE INDEX IF NOT EXISTS idx_pheno_embeddings_fts
ON pheno_embeddings USING GIN (to_tsvector('english', content));

-- Trigger for updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_pheno_embeddings_updated_at
    BEFORE UPDATE ON pheno_embeddings
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
"""
```

#### Vector Repository Pattern

```python
from dataclasses import dataclass
from typing import Optional, List, Dict, Any
from datetime import datetime
import asyncpg
from pgvector.asyncpg import register_vector
import numpy as np

@dataclass(frozen=True)
class SearchResult:
    """Result from vector similarity search."""
    id: str
    content: str
    content_type: str
    source_type: str
    source_path: str
    similarity: float
    metadata: Dict[str, Any]
    tags: List[str]

@dataclass
class SearchQuery:
    """Query parameters for semantic search."""
    query: str
    embedding: Optional[List[float]] = None
    content_types: Optional[List[str]] = None
    source_types: Optional[List[str]] = None
    tags: Optional[List[str]] = None
    project_id: Optional[str] = None
    top_k: int = 10
    similarity_threshold: float = 0.7
    use_hybrid: bool = True  # Combine vector + text search
    vector_weight: float = 0.7  # Weight for vector vs text score

class VectorRepository:
    """Repository for vector embeddings with hybrid search."""
    
    def __init__(self, pool: asyncpg.Pool, embedding_dim: int = 1536):
        self._pool = pool
        self._dim = embedding_dim
        self._metrics = VectorSearchMetrics()
    
    async def initialize(self) -> None:
        """Initialize vector extension and tables."""
        async with self._pool.acquire() as conn:
            await register_vector(conn)
            await conn.execute(VECTOR_SCHEMA)
    
    async def store(self,
                    content: str,
                    content_type: str,
                    embedding: List[float],
                    source_type: str = None,
                    source_path: str = None,
                    source_id: str = None,
                    metadata: dict = None,
                    tags: List[str] = None,
                    project_id: str = None) -> str:
        """Store content with embedding."""
        async with self._pool.acquire() as conn:
            row = await conn.fetchrow(
                """INSERT INTO pheno_embeddings 
                    (content, content_type, embedding, source_type, source_path, 
                     source_id, metadata, tags, project_id)
                   VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                   RETURNING id""",
                content, content_type, embedding, source_type, source_path,
                source_id, json.dumps(metadata or {}), tags or [], project_id
            )
            return str(row['id'])
    
    async def search(self, query: SearchQuery) -> List[SearchResult]:
        """Hybrid semantic search with optional text matching."""
        if query.use_hybrid and query.embedding:
            return await self._hybrid_search(query)
        elif query.embedding:
            return await self._vector_search(query)
        else:
            return await self._text_search(query)
    
    async def _vector_search(self, query: SearchQuery) -> List[SearchResult]:
        """Pure vector similarity search."""
        with self._metrics.measure("vector_search"):
            async with self._pool.acquire() as conn:
                rows = await conn.fetch(
                    """SELECT id, content, content_type, source_type, source_path,
                              metadata, tags,
                              1 - (embedding <=> $1) AS similarity
                       FROM pheno_embeddings
                       WHERE 1 - (embedding <=> $1) > $2
                         AND ($3::varchar[] IS NULL OR content_type = ANY($3))
                         AND ($4::varchar[] IS NULL OR source_type = ANY($4))
                         AND ($5::varchar[] IS NULL OR tags && $5)
                         AND ($6::varchar IS NULL OR project_id = $6)
                       ORDER BY embedding <=> $1
                       LIMIT $7""",
                    query.embedding,
                    query.similarity_threshold,
                    query.content_types,
                    query.source_types,
                    query.tags,
                    query.project_id,
                    query.top_k
                )
                
                return [self._row_to_result(row) for row in rows]
    
    async def _text_search(self, query: SearchQuery) -> List[SearchResult]:
        """Full-text search using Postgres tsvector."""
        with self._metrics.measure("text_search"):
            async with self._pool.acquire() as conn:
                rows = await conn.fetch(
                    """SELECT id, content, content_type, source_type, source_path,
                              metadata, tags,
                              ts_rank(to_tsvector('english', content), 
                                     plainto_tsquery('english', $1)) AS similarity
                       FROM pheno_embeddings
                       WHERE to_tsvector('english', content) 
                             @@ plainto_tsquery('english', $1)
                         AND ($2::varchar[] IS NULL OR content_type = ANY($2))
                         AND ($3::varchar[] IS NULL OR source_type = ANY($3))
                       ORDER BY similarity DESC
                       LIMIT $4""",
                    query.query,
                    query.content_types,
                    query.source_types,
                    query.top_k
                )
                
                return [self._row_to_result(row) for row in rows]
    
    async def _hybrid_search(self, query: SearchQuery) -> List[SearchResult]:
        """Combine vector and text search with weighted scores."""
        with self._metrics.measure("hybrid_search"):
            async with self._pool.acquire() as conn:
                rows = await conn.fetch(
                    """WITH vector_scores AS (
                        SELECT id, 
                               1 - (embedding <=> $1) AS vector_score
                        FROM pheno_embeddings
                        WHERE 1 - (embedding <=> $1) > $2
                          AND ($3::varchar[] IS NULL OR content_type = ANY($3))
                          AND ($4::varchar[] IS NULL OR source_type = ANY($4))
                          AND ($5::varchar[] IS NULL OR project_id = $5)
                        ORDER BY embedding <=> $1
                        LIMIT $6 * 2
                    ),
                    text_scores AS (
                        SELECT id,
                               ts_rank(to_tsvector('english', content),
                                      plainto_tsquery('english', $7)) AS text_score
                        FROM pheno_embeddings
                        WHERE to_tsvector('english', content) 
                              @@ plainto_tsquery('english', $7)
                          AND ($3::varchar[] IS NULL OR content_type = ANY($3))
                          AND ($4::varchar[] IS NULL OR source_type = ANY($4))
                          AND ($5::varchar IS NULL OR project_id = $5)
                    ),
                    combined AS (
                        SELECT 
                            e.id, e.content, e.content_type, e.source_type, 
                            e.source_path, e.metadata, e.tags,
                            COALESCE(v.vector_score, 0) AS vector_score,
                            COALESCE(t.text_score, 0) AS text_score,
                            ($8 * COALESCE(v.vector_score, 0) + 
                             (1 - $8) * COALESCE(t.text_score, 0)) AS hybrid_score
                        FROM pheno_embeddings e
                        LEFT JOIN vector_scores v ON e.id = v.id
                        LEFT JOIN text_scores t ON e.id = t.id
                        WHERE v.id IS NOT NULL OR t.id IS NOT NULL
                    )
                    SELECT * FROM combined
                    ORDER BY hybrid_score DESC
                    LIMIT $6""",
                    query.embedding,
                    query.similarity_threshold,
                    query.content_types,
                    query.source_types,
                    query.project_id,
                    query.top_k,
                    query.query,
                    query.vector_weight
                )
                
                return [self._row_to_result(row) for row in rows]
    
    async def find_similar(self, 
                          content_id: str,
                          top_k: int = 5,
                          exclude_self: bool = True) -> List[SearchResult]:
        """Find content similar to existing item."""
        async with self._pool.acquire() as conn:
            rows = await conn.fetch(
                """WITH target AS (
                    SELECT embedding FROM pheno_embeddings WHERE id = $1
                )
                SELECT e.id, e.content, e.content_type, e.source_type,
                       e.source_path, e.metadata, e.tags,
                       1 - (e.embedding <=> t.embedding) AS similarity
                FROM pheno_embeddings e
                CROSS JOIN target t
                WHERE e.id != $1 OR NOT $3
                ORDER BY e.embedding <=> t.embedding
                LIMIT $2""",
                content_id, top_k, exclude_self
            )
            
            return [self._row_to_result(row) for row in rows]
    
    def _row_to_result(self, row: asyncpg.Record) -> SearchResult:
        """Convert database row to SearchResult."""
        return SearchResult(
            id=str(row['id']),
            content=row['content'],
            content_type=row['content_type'],
            source_type=row['source_type'],
            source_path=row['source_path'],
            similarity=row.get('similarity', row.get('hybrid_score', 0)),
            metadata=json.loads(row['metadata']) if row['metadata'] else {},
            tags=row['tags'] or []
        )
```

#### Embedding Providers

```python
from typing import Protocol
import openai
import numpy as np

class Embedder(Protocol):
    """Protocol for text embedding providers."""
    
    async def embed(self, texts: List[str]) -> List[List[float]]:
        """Embed texts into vectors."""
        ...
    
    @property
    def dimension(self) -> int:
        """Return embedding dimension."""
        ...
    
    @property
    def max_tokens(self) -> int:
        """Return maximum tokens per input."""
        ...

class OpenAIEmbedder:
    """OpenAI embedding models."""
    
    MODELS = {
        "text-embedding-3-small": {"dim": 1536, "max_tokens": 8192},
        "text-embedding-3-large": {"dim": 3072, "max_tokens": 8192},
        "text-embedding-ada-002": {"dim": 1536, "max_tokens": 8191},
    }
    
    def __init__(self, 
                 model: str = "text-embedding-3-small",
                 api_key: str = None,
                 batch_size: int = 100):
        self._client = openai.AsyncOpenAI(api_key=api_key)
        self._model = model
        self._batch_size = batch_size
        self._info = self.MODELS[model]
    
    async def embed(self, texts: List[str]) -> List[List[float]]:
        """Embed texts in batches."""
        all_embeddings = []
        
        for i in range(0, len(texts), self._batch_size):
            batch = texts[i:i + self._batch_size]
            
            response = await self._client.embeddings.create(
                model=self._model,
                input=batch
            )
            
            embeddings = [e.embedding for e in response.data]
            all_embeddings.extend(embeddings)
        
        return all_embeddings
    
    @property
    def dimension(self) -> int:
        return self._info["dim"]
    
    @property
    def max_tokens(self) -> int:
        return self._info["max_tokens"]

class LocalEmbedder:
    """Local embedding with sentence-transformers."""
    
    def __init__(self, model_name: str = "all-MiniLM-L6-v2"):
        from sentence_transformers import SentenceTransformer
        self._model = SentenceTransformer(model_name)
        self._dim = self._model.get_sentence_embedding_dimension()
    
    async def embed(self, texts: List[str]) -> List[List[float]]:
        """Embed using local model (CPU-bound, run in thread)."""
        loop = asyncio.get_event_loop()
        embeddings = await loop.run_in_executor(
            None, self._model.encode, texts
        )
        return embeddings.tolist()
    
    @property
    def dimension(self) -> int:
        return self._dim
    
    @property
    def max_tokens(self) -> int:
        return 512  # Model-dependent

class HybridEmbedder:
    """Use OpenAI for quality, local for privacy-sensitive content."""
    
    def __init__(self, 
                 openai_embedder: OpenAIEmbedder,
                 local_embedder: LocalEmbedder,
                 privacy_patterns: List[str] = None):
        self._openai = openai_embedder
        self._local = local_embedder
        self._privacy_patterns = privacy_patterns or [
            r"password", r"secret", r"key", r"token", r"credential",
            r"private_key", r"api_key", r"auth"
        ]
    
    async def embed(self, texts: List[str]) -> List[List[float]]:
        """Route to appropriate embedder based on content."""
        import re
        
        openai_texts = []
        local_texts = []
        openai_indices = []
        local_indices = []
        
        for i, text in enumerate(texts):
            text_lower = text.lower()
            if any(re.search(p, text_lower) for p in self._privacy_patterns):
                local_texts.append(text)
                local_indices.append(i)
            else:
                openai_texts.append(text)
                openai_indices.append(i)
        
        # Embed in parallel
        openai_results, local_results = await asyncio.gather(
            self._openai.embed(openai_texts) if openai_texts else [],
            self._local.embed(local_texts) if local_texts else []
        )
        
        # Merge results
        embeddings = [None] * len(texts)
        for idx, emb in zip(openai_indices, openai_results):
            embeddings[idx] = emb
        for idx, emb in zip(local_indices, local_results):
            embeddings[idx] = emb
        
        return embeddings
```

#### Document Ingestion Pipeline

```python
class DocumentIngestor:
    """Ingest documents into vector store."""
    
    def __init__(self,
                 repository: VectorRepository,
                 embedder: Embedder,
                 chunker: Chunker):
        self._repo = repository
        self._embedder = embedder
        self._chunker = chunker
    
    async def ingest_file(self, 
                          file_path: Path,
                          content_type: str = "doc",
                          project_id: str = None) -> List[str]:
        """Ingest a file into the vector store."""
        # Read and parse file
        content = await self._read_file(file_path)
        
        # Chunk for large documents
        chunks = self._chunker.chunk(content)
        
        # Generate embeddings
        embeddings = await self._embedder.embed(chunks)
        
        # Store chunks
        ids = []
        for i, (chunk, embedding) in enumerate(zip(chunks, embeddings)):
            chunk_id = await self._repo.store(
                content=chunk,
                content_type=content_type,
                embedding=embedding,
                source_type=file_path.suffix.lstrip('.'),
                source_path=str(file_path),
                source_id=f"{file_path}:{i}",
                metadata={
                    "chunk_index": i,
                    "total_chunks": len(chunks),
                    "file_name": file_path.name,
                },
                project_id=project_id
            )
            ids.append(chunk_id)
        
        return ids
    
    async def ingest_directory(self,
                               directory: Path,
                               pattern: str = "**/*.md",
                               content_type: str = "doc",
                               project_id: str = None) -> Dict[str, List[str]]:
        """Ingest all matching files in directory."""
        results = {}
        
        files = list(directory.glob(pattern))
        
        # Process in batches with progress
        for i in range(0, len(files), 10):
            batch = files[i:i + 10]
            
            tasks = [
                self.ingest_file(f, content_type, project_id)
                for f in batch
            ]
            batch_results = await asyncio.gather(*tasks)
            
            for f, ids in zip(batch, batch_results):
                results[str(f)] = ids
        
        return results

class Chunker:
    """Chunk documents with overlap for context preservation."""
    
    def __init__(self,
                 chunk_size: int = 1000,
                 chunk_overlap: int = 200,
                 separator: str = "\n"):
        self.chunk_size = chunk_size
        self.chunk_overlap = chunk_overlap
        self.separator = separator
    
    def chunk(self, text: str) -> List[str]:
        """Split text into overlapping chunks."""
        if len(text) <= self.chunk_size:
            return [text]
        
        chunks = []
        start = 0
        
        while start < len(text):
            end = start + self.chunk_size
            
            # Try to break at separator
            if end < len(text):
                # Look for separator within overlap window
                search_start = max(start + self.chunk_size - self.chunk_overlap, start)
                sep_pos = text.rfind(self.separator, search_start, end + 100)
                if sep_pos != -1:
                    end = sep_pos + len(self.separator)
            
            chunks.append(text[start:end].strip())
            
            # Advance with overlap
            start = end - self.chunk_overlap
            if start <= 0:
                start = end
        
        return chunks
```

## Consequences

### Positive
- **Semantic Search**: Find content by meaning, not just keywords
- **AI Integration**: Foundation for RAG, recommendations, classification
- **Postgres Ecosystem**: Leverage existing infrastructure, backups, monitoring
- **Hybrid Search**: Combine vector similarity with text matching
- **Scalability**: HNSW indexes enable fast search on large datasets

### Negative
- **Postgres Dependency**: Requires pgvector-enabled Postgres
- **Embedding Costs**: OpenAI API usage for quality embeddings
- **Dimension Storage**: 1536d vectors increase storage requirements
- **Index Build Time**: HNSW construction can be slow for large datasets
- **Privacy**: External embedding sends content to third parties

### Mitigations

1. **Hybrid Embedding**: Local models for sensitive content
2. **Selective Indexing**: Only index high-value content
3. **Incremental Updates**: Update embeddings on change, not full rebuild
4. **Caching**: Cache embeddings for unchanged content
5. **Dimension Reduction**: Consider PCA for storage optimization if needed

## Implementation Plan

### Phase 1: Infrastructure (Week 1)
- [ ] Set up pgvector-enabled Postgres
- [ ] Create vector repository base class
- [ ] Implement OpenAI embedder
- [ ] Add local embedder option

### Phase 2: Ingestion (Week 2)
- [ ] Build document chunking pipeline
- [ ] Create file system watcher for auto-indexing
- [ ] Implement batch embedding with progress
- [ ] Add metadata extraction

### Phase 3: Search (Week 3)
- [ ] Implement hybrid search API
- [ ] Create search CLI command
- [ ] Add TUI search interface
- [ ] Implement similar content suggestions

### Phase 4: Integration (Week 4)
- [ ] Integrate with documentation viewer
- [ ] Add error suggestion feature
- [ ] Implement code example retrieval
- [ ] Create configuration assistant

## Alternatives Considered

### Alternative 1: Pinecone
- **Pros**: Managed service, excellent performance, no index tuning
- **Cons**: Vendor lock-in, external dependency, cost at scale
- **Verdict**: Rejected; prefer data sovereignty with Postgres

### Alternative 2: Chroma
- **Pros**: Simple API, embeddable, good for small scale
- **Cons**: Less mature, single-node only
- **Verdict**: Rejected; Postgres ecosystem more mature

### Alternative 3: Weaviate
- **Pros**: Purpose-built, multi-modal, GraphQL interface
- **Cons**: Additional infrastructure, learning curve
- **Verdict**: Rejected; prefer extending existing Postgres

## Related Decisions
- ADR-002: Domain-Driven Design (repository pattern)
- ADR-003: Structured Concurrency (async operations)
- ADR-004: TUI Architecture (search interface)

## References
- pgvector documentation: https://github.com/pgvector/pgvector
- OpenAI Embeddings: https://platform.openai.com/docs/guides/embeddings
- HNSW Algorithm: https://arxiv.org/abs/1603.09320
- sentence-transformers: https://www.sbert.net/
