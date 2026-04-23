# ADR-003: ML Model Integration

**Document ID:** PHENOTYPE_PHENOLANG_ADR-003  
**Status:** Proposed  
**Last Updated:** 2026-04-03  
**Author:** Phenotype Architecture Team  
**Supersedes:** N/A  
**Related:** [NLP_TOOLKITS_SOTA.md](../research/NLP_TOOLKITS_SOTA.md), [ADR-001-nlp-engine.md](ADR-001-nlp-engine.md), [ADR-002-tokenization.md](ADR-002-tokenization.md)

---

## Table of Contents

1. [Context](#context)
2. [Decision](#decision)
3. [Consequences](#consequences)
4. [Implementation Details](#implementation-details)
5. [Alternatives Considered](#alternatives-considered)
6. [Validation](#validation)
7. [References](#references)

---

## Context

### Problem Statement

PhenoLang needs a robust strategy for integrating machine learning models into the NLP pipeline. This includes:

- Embedding models (dense, sparse, hybrid)
- Classification models (text categorization, sentiment analysis)
- Generation models (text generation, summarization)
- Reranking models (search result reranking)
- Domain-specific models (medical, legal, technical)

The integration strategy must address:

1. **Model Loading**: Efficient loading and caching of ML models
2. **Inference**: High-performance inference with batching and optimization
3. **Model Management**: Versioning, updates, and rollback capabilities
4. **Resource Management**: GPU/CPU allocation, memory management
5. **Monitoring**: Model performance tracking, drift detection
6. **Security**: Model supply chain security, input validation

### Current State

ML model integration is currently ad-hoc across the Phenotype ecosystem:

```
┌─────────────────────────────────────────────────────────────────────┐
│              Current ML Model Integration State                     │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  pheno-vector/                                                      │
│  ├── embeddings/          # Hugging Face models (implicit)         │
│  └── providers/           # Multiple embedding providers           │
│                                                                     │
│  pheno-tools/                                                       │
│  ├── embedding_backfill.py  # One-off embedding script             │
│  └── shared/                # Shared utilities                     │
│                                                                     │
│  PROBLEM: No unified model management, inconsistent inference       │
│  patterns, no monitoring, no versioning, no resource management.    │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### Model Integration Requirements

| Requirement | Priority | Description |
|-------------|----------|-------------|
| Unified interface | High | Consistent API across all model types |
| Model versioning | High | Track model versions and enable rollback |
| Resource management | High | Efficient GPU/CPU utilization |
| Batching | High | Automatic batching for throughput |
| Monitoring | Medium | Track model performance and drift |
| Caching | Medium | Cache model outputs for repeated inputs |
| Security | Medium | Validate model sources and inputs |
| Multi-provider | Medium | Support local and cloud model providers |

### Constraints

1. **Python Version**: Must support Python 3.12+ (per project standards)
2. **Hardware**: Must support both CPU and GPU inference
3. **Memory**: Models must fit within available memory constraints
4. **Latency**: Inference latency must meet application requirements (<100ms p99)
5. **Throughput**: Must handle production-scale workloads (10K+ inferences/sec)
6. **Security**: Models must be from verified sources with integrity checks

---

## Decision

### Selected Approach: Unified Model Interface with Provider Abstraction

We will implement a **unified model interface** that abstracts over different model providers and types, with:

1. **Model Registry**: Centralized model management with versioning and lifecycle
2. **Provider Abstraction**: Consistent interface for local and cloud providers
3. **Inference Engine**: Optimized inference with batching, caching, and resource management
4. **Monitoring**: Built-in performance tracking and drift detection

### Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                    ML Model Integration Architecture                │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                    Model API Layer                          │   │
│  │  - EmbeddingModel (dense, sparse, hybrid)                  │   │
│  │  - ClassificationModel (text categorization)               │   │
│  │  - GenerationModel (text generation)                       │   │
│  │  - RerankingModel (search reranking)                       │   │
│  └───────────────────────────┬─────────────────────────────────┘   │
│                              │                                      │
│  ┌───────────────────────────▼─────────────────────────────────┐   │
│  │                  Inference Engine                           │   │
│  │  - Batching (automatic, configurable)                      │   │
│  │  - Caching (LRU, TTL-based)                                │   │
│  │  - Resource Management (GPU/CPU allocation)                │   │
│  │  - Queue Management (priority, fair scheduling)            │   │
│  └───────────────────────────┬─────────────────────────────────┘   │
│                              │                                      │
│  ┌───────────────────────────▼─────────────────────────────────┐   │
│  │                  Model Registry                             │   │
│  │  - Model Versioning (semantic versioning)                  │   │
│  │  - Model Lifecycle (load, unload, update, rollback)        │   │
│  │  - Model Metadata (performance, training data, license)    │   │
│  │  - Model Validation (integrity, compatibility)             │   │
│  └───────────────────────────┬─────────────────────────────────┘   │
│                              │                                      │
│  ┌───────────────────────────▼─────────────────────────────────┐   │
│  │                  Provider Layer                             │   │
│  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐   │   │
│  │  │   Local     │ │   Ollama    │ │   Cloud APIs        │   │   │
│  │  │  (torch)    │ │  (local)    │ │  (OpenAI, Cohere)   │   │   │
│  │  └─────────────┘ └─────────────┘ └─────────────────────┘   │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                  Monitoring Layer                           │   │
│  │  - Performance Metrics (latency, throughput, error rate)   │   │
│  │  - Model Drift Detection (input distribution, output dist) │   │   │
│  │  - Resource Monitoring (GPU memory, CPU usage)             │   │
│  │  - Alerting (threshold-based, anomaly detection)           │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### Model Interface

```python
"""ML model interface definitions."""

from typing import Protocol, List, Dict, Any, Optional
from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from enum import Enum
import time


class ModelType(Enum):
    """Types of ML models."""
    EMBEDDING = "embedding"
    CLASSIFICATION = "classification"
    GENERATION = "generation"
    RERANKING = "reranking"
    CUSTOM = "custom"


class ModelProvider(Enum):
    """Model providers."""
    LOCAL = "local"
    OLLAMA = "ollama"
    OPENAI = "openai"
    COHERE = "cohere"
    CUSTOM = "custom"


@dataclass
class ModelMetadata:
    """Metadata for ML models."""
    name: str
    version: str
    model_type: ModelType
    provider: ModelProvider
    description: str = ""
    license: str = ""
    training_data: str = ""
    performance_metrics: Dict[str, float] = field(default_factory=dict)
    tags: List[str] = field(default_factory=list)
    created_at: str = ""
    updated_at: str = ""


@dataclass
class InferenceResult:
    """Result from model inference."""
    model_name: str
    model_version: str
    data: Any
    metadata: Dict[str, Any] = field(default_factory=dict)
    inference_time: float = 0.0
    error: Optional[str] = None


class MLModel(Protocol):
    """Protocol for ML model implementations."""

    metadata: ModelMetadata

    def predict(self, input_data: Any, **kwargs) -> InferenceResult:
        """Run inference on input data."""
        ...

    def predict_batch(self, inputs: List[Any], **kwargs) -> List[InferenceResult]:
        """Run inference on batch of inputs."""
        ...

    def load(self) -> None:
        """Load model into memory."""
        ...

    def unload(self) -> None:
        """Unload model from memory."""
        ...


class EmbeddingModel(Protocol):
    """Protocol for embedding models."""

    metadata: ModelMetadata
    dimensions: int

    def encode(self, texts: List[str], **kwargs) -> InferenceResult:
        """Encode texts to embeddings."""
        ...

    def similarity(self, text1: str, text2: str) -> float:
        """Compute similarity between two texts."""
        ...

    def most_similar(
        self,
        query: str,
        candidates: List[str],
        top_k: int = 5
    ) -> List[tuple[str, float]]:
        """Find most similar candidates to query."""
        ...
```

### Model Registry

```python
"""Model registry for managing ML models."""

from typing import Dict, List, Optional
from pheno_nlp.models.base import MLModel, ModelMetadata, ModelType


class ModelRegistry:
    """Registry for ML models."""

    def __init__(self):
        self._models: Dict[str, MLModel] = {}
        self._model_versions: Dict[str, List[ModelMetadata]] = {}
        self._active_models: Dict[str, str] = {}  # model_name -> version

    def register(self, model: MLModel) -> None:
        """Register a model."""
        key = f"{model.metadata.name}:{model.metadata.version}"
        self._models[key] = model

        if model.metadata.name not in self._model_versions:
            self._model_versions[model.metadata.name] = []
        self._model_versions[model.metadata.name].append(model.metadata)

    def get(self, name: str, version: Optional[str] = None) -> MLModel:
        """Get a model by name and optional version."""
        if version:
            key = f"{name}:{version}"
        else:
            # Get active version
            active_version = self._active_models.get(name)
            if not active_version:
                raise KeyError(f"No active version for model: {name}")
            key = f"{name}:{active_version}"

        if key not in self._models:
            raise KeyError(f"Model not found: {key}")

        return self._models[key]

    def activate(self, name: str, version: str) -> None:
        """Activate a specific model version."""
        key = f"{name}:{version}"
        if key not in self._models:
            raise KeyError(f"Model not found: {key}")

        self._active_models[name] = version

    def list_models(self, model_type: Optional[ModelType] = None) -> List[ModelMetadata]:
        """List all registered models, optionally filtered by type."""
        models = []
        for versions in self._model_versions.values():
            for metadata in versions:
                if model_type is None or metadata.model_type == model_type:
                    models.append(metadata)
        return models

    def list_versions(self, name: str) -> List[str]:
        """List all versions of a model."""
        if name not in self._model_versions:
            return []
        return [m.version for m in self._model_versions[name]]

    def rollback(self, name: str, version: str) -> None:
        """Rollback to a previous model version."""
        self.activate(name, version)
```

### Inference Engine

```python
"""Inference engine with batching and caching."""

from typing import Any, List, Dict, Optional
from collections import OrderedDict
import asyncio
import time


class InferenceCache:
    """LRU cache for inference results."""

    def __init__(self, max_size: int = 10000, ttl: float = 3600.0):
        self.max_size = max_size
        self.ttl = ttl
        self._cache: OrderedDict[str, tuple[Any, float]] = OrderedDict()

    def get(self, key: str) -> Optional[Any]:
        """Get cached result."""
        if key in self._cache:
            result, timestamp = self._cache[key]
            if time.time() - timestamp < self.ttl:
                # Move to end (most recently used)
                self._cache.move_to_end(key)
                return result
            else:
                # Expired
                del self._cache[key]
        return None

    def put(self, key: str, value: Any) -> None:
        """Cache a result."""
        if key in self._cache:
            self._cache.move_to_end(key)
        self._cache[key] = (value, time.time())

        if len(self._cache) > self.max_size:
            self._cache.popitem(last=False)

    def clear(self) -> None:
        """Clear the cache."""
        self._cache.clear()


class BatchingEngine:
    """Automatic batching for inference."""

    def __init__(self, max_batch_size: int = 32, max_wait_ms: float = 100.0):
        self.max_batch_size = max_batch_size
        self.max_wait_ms = max_wait_ms
        self._queue: asyncio.Queue = asyncio.Queue()
        self._running = False

    async def start(self) -> None:
        """Start the batching engine."""
        self._running = True
        asyncio.create_task(self._process_batches())

    async def stop(self) -> None:
        """Stop the batching engine."""
        self._running = False

    async def submit(self, input_data: Any) -> Any:
        """Submit input for batched inference."""
        future = asyncio.get_event_loop().create_future()
        await self._queue.put((input_data, future))
        return await future

    async def _process_batches(self) -> None:
        """Process batches from the queue."""
        while self._running:
            batch = []
            futures = []

            # Collect batch
            start_time = time.time()
            while len(batch) < self.max_batch_size:
                elapsed_ms = (time.time() - start_time) * 1000
                if elapsed_ms >= self.max_wait_ms:
                    break

                try:
                    input_data, future = await asyncio.wait_for(
                        self._queue.get(), timeout=0.01
                    )
                    batch.append(input_data)
                    futures.append(future)
                except asyncio.TimeoutError:
                    continue

            if batch:
                # Process batch (to be implemented by subclass)
                results = await self._process_batch(batch)

                # Return results
                for future, result in zip(futures, results):
                    if not future.done():
                        future.set_result(result)

    async def _process_batch(self, batch: List[Any]) -> List[Any]:
        """Process a batch of inputs. Override in subclass."""
        raise NotImplementedError
```

---

## Consequences

### Positive Consequences

1. **Unified Interface**: All ML models use a consistent interface, making it easy to swap models without changing downstream code.

2. **Model Versioning**: Built-in versioning enables safe model updates and easy rollback to previous versions.

3. **Resource Optimization**: Automatic batching and caching improve throughput and reduce inference latency.

4. **Multi-Provider Support**: Support for local and cloud providers enables flexibility in deployment and cost optimization.

5. **Monitoring**: Built-in performance tracking enables proactive identification of model degradation and resource issues.

6. **Security**: Model validation and input sanitization protect against model supply chain attacks and adversarial inputs.

7. **Scalability**: The architecture supports horizontal scaling through distributed model serving.

8. **Developer Experience**: Clear interfaces and comprehensive documentation reduce the learning curve for integrating ML models.

### Negative Consequences

1. **Complexity**: The abstraction layers add complexity compared to direct model usage.

2. **Overhead**: The inference engine adds some latency overhead compared to direct model calls.

3. **Memory Usage**: Caching and model loading increase memory consumption.

4. **Operational Complexity**: Managing model versions, updates, and monitoring adds operational overhead.

5. **Testing Burden**: Multiple model providers and versions require comprehensive testing.

6. **Dependency Management**: Managing dependencies for multiple ML frameworks (torch, transformers, etc.) can be complex.

### Mitigation Strategies

| Risk | Mitigation |
|------|-----------|
| Complexity | Clear interfaces, comprehensive documentation, examples |
| Overhead | Benchmark and optimize critical paths, configurable batching |
| Memory usage | Configurable cache sizes, model unloading, memory limits |
| Operational complexity | Automated deployment, monitoring dashboards, alerting |
| Testing burden | Integration test suites, mock providers, benchmark suites |
| Dependency management | uv for dependency resolution, version pinning |

---

## Implementation Details

### Model Provider Implementations

```python
"""Local model provider implementation."""

from typing import List, Optional
import numpy as np

from pheno_nlp.models.base import (
    EmbeddingModel, ModelMetadata, ModelType, ModelProvider, InferenceResult
)


class LocalEmbeddingModel:
    """Local embedding model using sentence-transformers."""

    def __init__(
        self,
        model_name: str,
        device: str = "cpu",
        cache: Optional[InferenceCache] = None
    ):
        from sentence_transformers import SentenceTransformer
        self.model_name = model_name
        self.device = device
        self.cache = cache or InferenceCache()
        self._model: Optional[SentenceTransformer] = None

        self.metadata = ModelMetadata(
            name=model_name,
            version="1.0.0",
            model_type=ModelType.EMBEDDING,
            provider=ModelProvider.LOCAL,
            description=f"Local embedding model: {model_name}",
        )

    def load(self) -> None:
        """Load the model into memory."""
        if self._model is None:
            from sentence_transformers import SentenceTransformer
            self._model = SentenceTransformer(self.model_name, device=self.device)

    @property
    def dimensions(self) -> int:
        """Get embedding dimensions."""
        self.load()
        return self._model.get_sentence_embedding_dimension()

    def encode(self, texts: List[str], **kwargs) -> InferenceResult:
        """Encode texts to embeddings."""
        import time
        start = time.perf_counter()

        # Check cache
        cache_key = str(texts)
        cached = self.cache.get(cache_key)
        if cached is not None:
            return InferenceResult(
                model_name=self.model_name,
                model_version=self.metadata.version,
                data=cached,
                metadata={"cached": True},
            )

        self.load()
        embeddings = self._model.encode(texts, **kwargs)

        # Cache result
        self.cache.put(cache_key, embeddings)

        return InferenceResult(
            model_name=self.model_name,
            model_version=self.metadata.version,
            data=embeddings,
            metadata={"num_texts": len(texts)},
            inference_time=time.perf_counter() - start,
        )

    def similarity(self, text1: str, text2: str) -> float:
        """Compute similarity between two texts."""
        result = self.encode([text1, text2])
        embeddings = result.data
        return float(np.dot(embeddings[0], embeddings[1]))

    def most_similar(
        self,
        query: str,
        candidates: List[str],
        top_k: int = 5
    ) -> List[tuple[str, float]]:
        """Find most similar candidates to query."""
        all_texts = [query] + candidates
        result = self.encode(all_texts)
        embeddings = result.data

        query_embedding = embeddings[0]
        candidate_embeddings = embeddings[1:]

        similarities = np.dot(candidate_embeddings, query_embedding)
        indexed_sims = list(enumerate(similarities))
        indexed_sims.sort(key=lambda x: x[1], reverse=True)

        return [(candidates[i], float(sim)) for i, sim in indexed_sims[:top_k]]
```

### Cloud Provider Integration

```python
"""OpenAI provider implementation."""

from typing import List, Optional
import numpy as np

from pheno_nlp.models.base import (
    EmbeddingModel, ModelMetadata, ModelType, ModelProvider, InferenceResult
)


class OpenAIEmbeddingModel:
    """OpenAI embedding model."""

    def __init__(
        self,
        model_name: str = "text-embedding-3-small",
        api_key: Optional[str] = None,
        cache: Optional[InferenceCache] = None
    ):
        from openai import OpenAI
        self.model_name = model_name
        self.cache = cache or InferenceCache()
        self.client = OpenAI(api_key=api_key)

        self.metadata = ModelMetadata(
            name=model_name,
            version="1.0.0",
            model_type=ModelType.EMBEDDING,
            provider=ModelProvider.OPENAI,
            description=f"OpenAI embedding model: {model_name}",
        )

        # Dimension mapping for OpenAI models
        self._dimensions = {
            "text-embedding-3-small": 1536,
            "text-embedding-3-large": 3072,
            "text-embedding-ada-002": 1536,
        }

    @property
    def dimensions(self) -> int:
        """Get embedding dimensions."""
        return self._dimensions.get(self.model_name, 1536)

    def encode(self, texts: List[str], **kwargs) -> InferenceResult:
        """Encode texts using OpenAI API."""
        import time
        start = time.perf_counter()

        # Check cache
        cache_key = str(texts)
        cached = self.cache.get(cache_key)
        if cached is not None:
            return InferenceResult(
                model_name=self.model_name,
                model_version=self.metadata.version,
                data=cached,
                metadata={"cached": True},
            )

        response = self.client.embeddings.create(
            model=self.model_name,
            input=texts,
            **kwargs
        )

        embeddings = np.array([d.embedding for d in response.data])

        # Cache result
        self.cache.put(cache_key, embeddings)

        return InferenceResult(
            model_name=self.model_name,
            model_version=self.metadata.version,
            data=embeddings,
            metadata={
                "num_texts": len(texts),
                "usage": response.usage.dict() if response.usage else {},
            },
            inference_time=time.perf_counter() - start,
        )

    def similarity(self, text1: str, text2: str) -> float:
        """Compute similarity between two texts."""
        result = self.encode([text1, text2])
        embeddings = result.data
        return float(np.dot(embeddings[0], embeddings[1]))

    def most_similar(
        self,
        query: str,
        candidates: List[str],
        top_k: int = 5
    ) -> List[tuple[str, float]]:
        """Find most similar candidates to query."""
        all_texts = [query] + candidates
        result = self.encode(all_texts)
        embeddings = result.data

        query_embedding = embeddings[0]
        candidate_embeddings = embeddings[1:]

        similarities = np.dot(candidate_embeddings, query_embedding)
        indexed_sims = list(enumerate(similarities))
        indexed_sims.sort(key=lambda x: x[1], reverse=True)

        return [(candidates[i], float(sim)) for i, sim in indexed_sims[:top_k]]
```

### Integration with pheno-vector

```python
"""Integration with pheno-vector package."""

from pheno_vector.client import VectorClient
from pheno_nlp.models.registry import ModelRegistry
from pheno_nlp.models.embedding import LocalEmbeddingModel


def setup_vector_with_embeddings(
    vector_client: VectorClient,
    model_registry: ModelRegistry,
    model_name: str = "BAAI/bge-small-en-v1.5"
) -> None:
    """Configure vector client with embedding model."""
    # Get embedding model from registry
    embedding_model = model_registry.get(model_name)

    # Configure vector client
    vector_client.configure_embedder(
        model=embedding_model,
        dimensions=embedding_model.dimensions,
        batch_size=32,
    )


async def embed_and_store(
    vector_client: VectorClient,
    model_registry: ModelRegistry,
    documents: List[dict],
    model_name: str = "BAAI/bge-small-en-v1.5"
) -> None:
    """Embed documents and store in vector database."""
    embedding_model = model_registry.get(model_name)

    texts = [doc["text"] for doc in documents]
    ids = [doc["id"] for doc in documents]
    metadata = [doc.get("metadata", {}) for doc in documents]

    # Encode texts
    result = embedding_model.encode(texts)

    # Store in vector database
    await vector_client.add(
        ids=ids,
        vectors=result.data,
        metadata=metadata,
    )
```

---

## Alternatives Considered

### Alternative 1: Direct Model Usage

**Description**: Use models directly without abstraction layers.

**Pros**:
- Simpler architecture
- No overhead from abstraction layers
- Direct access to model features

**Cons**:
- Tight coupling to specific models
- No versioning or rollback
- No caching or batching
- Hard to swap providers

**Decision**: Rejected - lack of flexibility and operational features.

### Alternative 2: MLflow for Model Management

**Description**: Use MLflow for model management and serving.

**Pros**:
- Comprehensive ML lifecycle management
- Built-in model registry
- Experiment tracking

**Cons**:
- Heavy infrastructure requirements
- Overkill for PhenoLang's needs
- Additional operational complexity

**Decision**: Rejected - too heavy for current requirements, can adopt later.

### Alternative 3: BentoML for Model Serving

**Description**: Use BentoML for model serving and management.

**Pros**:
- Purpose-built for model serving
- Good performance
- Built-in batching and caching

**Cons**:
- Additional dependency
- Learning curve
- May be overkill for current needs

**Decision**: Deferred - can adopt if scaling requirements increase.

---

## Validation

### Success Criteria

1. **Performance**: Inference latency <100ms p99 for embedding models
2. **Throughput**: 10K+ inferences/second for batch processing
3. **Model Management**: Support 5+ concurrent model versions
4. **Cache Hit Rate**: >50% cache hit rate for repeated inputs
5. **Type Safety**: 100% type coverage for model interfaces
6. **Test Coverage**: >90% code coverage for model components

### Validation Plan

1. **Unit Tests**: Test individual model components in isolation
2. **Integration Tests**: Test model integration with pipeline and vector store
3. **Performance Tests**: Benchmark inference latency and throughput
4. **Load Tests**: Test under production-like load
5. **Rollback Tests**: Test model version rollback functionality

### Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Inference Latency (p99) | <100ms | APM monitoring |
| Throughput | 10K inferences/sec | Benchmark suite |
| Cache Hit Rate | >50% | Cache metrics |
| Model Versions | 5+ concurrent | Registry metrics |
| Type Coverage | 100% | mypy strict mode |
| Test Coverage | >90% | pytest-cov |

---

## References

1. [NLP_TOOLKITS_SOTA.md](../research/NLP_TOOLKITS_SOTA.md) - State-of-the-art research on ML models
2. [ADR-001-nlp-engine.md](ADR-001-nlp-engine.md) - NLP engine selection decision
3. [ADR-002-tokenization.md](ADR-002-tokenization.md) - Tokenization strategy decision
4. Hugging Face Transformers: https://huggingface.co/docs/transformers
5. Sentence Transformers: https://www.sbert.net/
6. OpenAI Embeddings: https://platform.openai.com/docs/guides/embeddings
7. MLflow: https://mlflow.org/
8. BentoML: https://www.bentoml.com/

---

*This ADR is proposed for review by the Phenotype Architecture Team. Decision pending stakeholder feedback.*
