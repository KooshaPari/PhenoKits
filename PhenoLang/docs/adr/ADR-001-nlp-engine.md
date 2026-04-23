# ADR-001: NLP Engine Selection

**Document ID:** PHENOTYPE_PHENOLANG_ADR-001  
**Status:** Accepted  
**Last Updated:** 2026-04-03  
**Author:** Phenotype Architecture Team  
**Supersedes:** N/A  
**Related:** [NLP_TOOLKITS_SOTA.md](../research/NLP_TOOLKITS_SOTA.md), [ADR-002-tokenization.md](ADR-002-tokenization.md), [ADR-003-model-integration.md](ADR-003-model-integration.md)

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

PhenoLang requires a robust, extensible NLP engine that can serve as the foundation for all language processing tasks within the Phenotype ecosystem. The engine must support:

- Multiple NLP tasks (tokenization, parsing, embedding, classification, generation)
- Pluggable backends (spaCy, Hugging Face, custom models)
- High performance for production workloads
- Type-safe interfaces for developer experience
- Streaming and batch processing modes
- Multilingual capabilities
- Integration with the broader Phenotype architecture (pheno-core, pheno-vector, pheno-tools)

### Current State

The Phenotype ecosystem currently has scattered NLP capabilities across multiple packages:

```
┌─────────────────────────────────────────────────────────────────────┐
│              Current NLP Capability Distribution                    │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  pheno-vector/                                                      │
│  ├── embeddings/          # Embedding computation                   │
│  ├── pipelines/           # Processing pipelines                    │
│  └── search/              # Semantic search                         │
│                                                                     │
│  pheno-tools/                                                       │
│  ├── embedding_backfill.py  # Embedding utilities                   │
│  └── shared/                # Shared utilities                      │
│                                                                     │
│  pheno-core/                                                        │
│  ├── stream.py              # Stream processing                     │
│  └── utils/                 # General utilities                     │
│                                                                     │
│  PROBLEM: No unified NLP engine, duplicated logic, inconsistent     │
│  interfaces, no type safety across packages.                        │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### Constraints

1. **Python Version**: Must support Python 3.12+ (per project standards)
2. **Performance**: Must handle production-scale workloads (10K+ requests/sec)
3. **Memory**: Must operate within reasonable memory constraints (<2GB per instance)
4. **Licensing**: All dependencies must be compatible with MIT licensing
5. **Integration**: Must integrate with existing Phenotype packages (pheno-core, pheno-vector)
6. **Maintainability**: Must have clear ownership and active maintenance

### Stakeholders

- **Phenotype Architecture Team**: Primary consumers and maintainers
- **Application Teams**: End users of NLP capabilities
- **DevOps**: Operational concerns (deployment, monitoring, scaling)
- **Security Team**: Data privacy and model security requirements

---

## Decision

### Selected Approach: Hybrid Plugin Architecture with spaCy Core

We will implement a **hybrid plugin architecture** with **spaCy as the primary NLP pipeline engine**, supplemented by **Hugging Face Transformers for embedding and generation tasks**, and a **custom plugin registry** for extensibility.

### Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                      PhenoLang NLP Engine                           │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                    Public API Layer                         │   │
│  │  - NLPipeline (composable pipeline)                        │   │
│  │  - NLPProcessor (unified processor)                        │   │
│  │  - NLPConfig (configuration management)                    │   │
│  └───────────────────────────┬─────────────────────────────────┘   │
│                              │                                      │
│  ┌───────────────────────────▼─────────────────────────────────┐   │
│  │                   Plugin Registry                           │   │
│  │  - TokenizerPlugin (spaCy, SentencePiece, Tiktoken)        │   │
│  │  - EmbedderPlugin (SentenceTransformers, OpenAI, Cohere)   │   │
│  │  - ParserPlugin (spaCy, Stanza, Benepar)                   │   │
│  │  - ClassifierPlugin (spaCy, sklearn, transformers)         │   │
│  │  - GeneratorPlugin (OpenAI, Anthropic, Ollama)             │   │
│  └───────────────────────────┬─────────────────────────────────┘   │
│                              │                                      │
│  ┌───────────────────────────▼─────────────────────────────────┐   │
│  │                    Core Engine                              │   │
│  │  - spaCy pipeline (primary processing)                     │   │
│  │  - Hugging Face transformers (embeddings, generation)      │   │
│  │  - Custom components (domain-specific processing)          │   │
│  └───────────────────────────┬─────────────────────────────────┘   │
│                              │                                      │
│  ┌───────────────────────────▼─────────────────────────────────┐   │
│  │                  Infrastructure Layer                       │   │
│  │  - pheno-core (registry, correlation, streaming)           │   │
│  │  - pheno-vector (embedding storage, retrieval)             │   │
│  │  - pheno-logging (observability, tracing)                  │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### Key Design Decisions

1. **spaCy as Primary Pipeline Engine**
   - spaCy provides the most production-ready NLP pipeline
   - Excellent type safety and modern Python practices
   - Strong community and active development
   - Comprehensive language support (70+ languages)
   - Built-in support for custom components

2. **Hugging Face for Embeddings and Generation**
   - Largest collection of pre-trained models
   - Excellent model hub and community
   - Support for multiple frameworks (PyTorch, TensorFlow, JAX)
   - Active research integration

3. **Custom Plugin Registry**
   - Enables easy integration of new backends
   - Provides consistent interfaces across different implementations
   - Allows domain-specific customizations
   - Supports hot-swapping of components

### Implementation Strategy

```python
"""PhenoLang NLP Engine - Core Implementation"""

from typing import Protocol, List, Dict, Optional, Any
from dataclasses import dataclass, field
from enum import Enum
import time

from pydantic import BaseModel, Field


class NLPTask(Enum):
    """Supported NLP tasks."""
    TOKENIZATION = "tokenization"
    EMBEDDING = "embedding"
    PARSING = "parsing"
    NER = "ner"
    CLASSIFICATION = "classification"
    GENERATION = "generation"


@dataclass
class NLPResult:
    """Result from NLP processing."""
    task: NLPTask
    data: Any
    metadata: Dict[str, Any] = field(default_factory=dict)
    processing_time: float = 0.0
    error: Optional[str] = None


class NLPComponent(Protocol):
    """Protocol for NLP components."""

    name: str
    version: str
    supported_tasks: List[NLPTask]

    def process(self, text: str, **kwargs) -> NLPResult:
        """Process text and return result."""
        ...

    def batch_process(self, texts: List[str], **kwargs) -> List[NLPResult]:
        """Process multiple texts."""
        ...


class SpaCyComponent:
    """spaCy-based NLP component."""

    name = "spacy"
    version = "1.0.0"
    supported_tasks = [
        NLPTask.TOKENIZATION,
        NLPTask.PARSING,
        NLPTask.NER,
        NLPTask.CLASSIFICATION,
    ]

    def __init__(self, model: str = "en_core_web_trf"):
        import spacy
        self.nlp = spacy.load(model)
        self._model = model

    def process(self, text: str, **kwargs) -> NLPResult:
        """Process text using spaCy pipeline."""
        start_time = time.perf_counter()

        try:
            doc = self.nlp(text)
            result = NLPResult(
                task=NLPTask.TOKENIZATION,
                data={
                    "tokens": [token.text for token in doc],
                    "pos_tags": [(token.text, token.pos_) for token in doc],
                    "entities": [(ent.text, ent.label_) for ent in doc.ents],
                    "dependencies": [
                        (token.text, token.dep_, token.head.text)
                        for token in doc
                    ],
                },
                metadata={
                    "model": self._model,
                    "num_tokens": len(doc),
                    "num_sentences": len(list(doc.sents)),
                },
                processing_time=time.perf_counter() - start_time,
            )
            return result

        except Exception as e:
            return NLPResult(
                task=NLPTask.TOKENIZATION,
                data=None,
                error=str(e),
                processing_time=time.perf_counter() - start_time,
            )

    def batch_process(self, texts: List[str], **kwargs) -> List[NLPResult]:
        """Process multiple texts using spaCy pipeline."""
        docs = self.nlp.pipe(texts)
        return [self._doc_to_result(doc) for doc in docs]

    def _doc_to_result(self, doc) -> NLPResult:
        """Convert spaCy Doc to NLPResult."""
        return NLPResult(
            task=NLPTask.TOKENIZATION,
            data={
                "tokens": [token.text for token in doc],
                "pos_tags": [(token.text, token.pos_) for token in doc],
                "entities": [(ent.text, ent.label_) for ent in doc.ents],
            },
            metadata={
                "model": self._model,
                "num_tokens": len(doc),
                "num_sentences": len(list(doc.sents)),
            },
        )


class NLPipeline:
    """Composable NLP pipeline."""

    def __init__(self, name: str = "default"):
        self.name = name
        self.components: List[NLPComponent] = []
        self._config: Dict[str, Any] = {}

    def add_component(self, component: NLPComponent) -> "NLPipeline":
        """Add a component to the pipeline."""
        self.components.append(component)
        return self

    def process(self, text: str) -> Dict[str, NLPResult]:
        """Process text through all pipeline components."""
        results = {}
        for component in self.components:
            for task in component.supported_tasks:
                result = component.process(text, task=task)
                results[f"{component.name}:{task.value}"] = result
        return results

    def configure(self, **kwargs) -> "NLPipeline":
        """Configure the pipeline."""
        self._config.update(kwargs)
        return self
```

---

## Consequences

### Positive Consequences

1. **Unified Interface**: All NLP operations go through a consistent, type-safe interface, reducing cognitive load for developers and enabling easier testing and maintenance.

2. **Pluggable Architecture**: The plugin system allows easy integration of new backends without modifying core code, enabling rapid experimentation and technology adoption.

3. **Production Ready**: spaCy's production-ready pipeline ensures reliable performance under load, with built-in support for batching, streaming, and error handling.

4. **Type Safety**: Pydantic-based configuration and data models provide compile-time type checking, reducing runtime errors and improving developer experience.

5. **Ecosystem Integration**: Direct integration with pheno-core (registry, streaming), pheno-vector (embeddings), and pheno-logging (observability) creates a cohesive architecture.

6. **Multilingual Support**: spaCy's 70+ language models and Hugging Face's multilingual models ensure comprehensive language coverage for the Phenotype ecosystem.

7. **Performance**: spaCy's optimized Cython implementation and Hugging Face's efficient transformers provide excellent performance for production workloads.

8. **Community Support**: Both spaCy and Hugging Face have large, active communities ensuring long-term viability and rapid bug fixes.

### Negative Consequences

1. **Dependency Complexity**: Multiple heavy dependencies (spaCy, transformers, torch) increase the installation footprint and potential for version conflicts.

2. **Learning Curve**: Developers need to understand both spaCy's pipeline architecture and Hugging Face's model hub, increasing onboarding time.

3. **Memory Overhead**: Loading multiple models (spaCy + transformers) can consume significant memory, requiring careful resource management in production.

4. **Version Lock-in**: Tightly coupling to specific versions of spaCy and transformers may complicate future upgrades and require coordinated migrations.

5. **Operational Complexity**: Managing multiple model deployments, updates, and monitoring adds operational overhead compared to a single-provider solution.

6. **Cold Start Latency**: Loading large models at startup can introduce significant cold start latency, requiring warm-up strategies or model caching.

### Mitigation Strategies

| Risk | Mitigation |
|------|-----------|
| Dependency complexity | Use uv for fast, reliable dependency resolution |
| Learning curve | Comprehensive documentation and examples |
| Memory overhead | Lazy model loading, model sharing, memory limits |
| Version lock-in | Abstract interfaces, version pinning, migration guides |
| Operational complexity | Automated deployment, monitoring, alerting |
| Cold start latency | Model preloading, warm-up endpoints, caching |

---

## Implementation Details

### Package Structure

```
pheno-nlp/
├── pyproject.toml
├── README.md
├── src/
│   └── pheno_nlp/
│       ├── __init__.py
│       ├── config.py           # Configuration management
│       ├── pipeline.py         # Pipeline orchestration
│       ├── registry.py         # Plugin registry
│       ├── components/
│       │   ├── __init__.py
│       │   ├── tokenizer.py    # Tokenization components
│       │   ├── embedder.py     # Embedding components
│       │   ├── parser.py       # Parsing components
│       │   ├── classifier.py   # Classification components
│       │   └── generator.py    # Generation components
│       ├── models/
│       │   ├── __init__.py
│       │   ├── result.py       # Result data models
│       │   └── config.py       # Configuration models
│       └── utils/
│           ├── __init__.py
│           └── validation.py   # Input validation
└── tests/
    ├── test_pipeline.py
    ├── test_components.py
    └── test_integration.py
```

### Configuration Management

```python
from pydantic import BaseModel, Field
from typing import Optional, Dict, Any


class SpaCyConfig(BaseModel):
    """Configuration for spaCy components."""
    model: str = Field(default="en_core_web_trf", description="spaCy model to use")
    disable: list[str] = Field(default_factory=list, description="Pipeline components to disable")
    exclude: list[str] = Field(default_factory=list, description="Doc attributes to exclude")
    config: Dict[str, Any] = Field(default_factory=dict, description="Additional spaCy config")


class EmbeddingConfig(BaseModel):
    """Configuration for embedding components."""
    model: str = Field(default="BAAI/bge-small-en-v1.5", description="Embedding model")
    device: str = Field(default="cpu", description="Device to run on (cpu, cuda)")
    batch_size: int = Field(default=32, ge=1, le=1024, description="Batch size for encoding")
    normalize: bool = Field(default=True, description="Normalize embeddings")
    max_length: int = Field(default=512, ge=1, le=8192, description="Maximum sequence length")


class NLPConfig(BaseModel):
    """Top-level NLP configuration."""
    spacy: SpaCyConfig = Field(default_factory=SpaCyConfig)
    embedding: EmbeddingConfig = Field(default_factory=EmbeddingConfig)
    pipeline: Dict[str, Any] = Field(default_factory=dict, description="Pipeline configuration")
    logging: Dict[str, Any] = Field(default_factory=dict, description="Logging configuration")

    @classmethod
    def from_env(cls) -> "NLPConfig":
        """Load configuration from environment variables."""
        import os
        return cls(
            spacy=SpaCyConfig(
                model=os.getenv("PHENO_NLP_SPACY_MODEL", "en_core_web_trf"),
            ),
            embedding=EmbeddingConfig(
                model=os.getenv("PHENO_NLP_EMBEDDING_MODEL", "BAAI/bge-small-en-v1.5"),
                device=os.getenv("PHENO_NLP_DEVICE", "cpu"),
            ),
        )
```

### Integration with pheno-core

```python
"""Integration with pheno-core registry system."""

from pheno_core.registry import Registry
from pheno_core.stream import StreamProcessor

from pheno_nlp.pipeline import NLPipeline
from pheno_nlp.components.tokenizer import SpaCyTokenizer
from pheno_nlp.components.embedder import EmbeddingComponent


def register_nlp_components(registry: Registry):
    """Register NLP components with the core registry."""
    # Register tokenizer
    registry.register(
        "nlp.tokenizer.spacy",
        SpaCyTokenizer(model="en_core_web_trf"),
    )

    # Register embedder
    registry.register(
        "nlp.embedder.bge",
        EmbeddingComponent(model="BAAI/bge-small-en-v1.5"),
    )

    # Register pipeline
    pipeline = NLPipeline(name="default")
    pipeline.add_component(registry.get("nlp.tokenizer.spacy"))
    pipeline.add_component(registry.get("nlp.embedder.bge"))

    registry.register("nlp.pipeline.default", pipeline)


def create_nlp_stream(registry: Registry) -> StreamProcessor:
    """Create a streaming NLP processor."""
    pipeline = registry.get("nlp.pipeline.default")

    return StreamProcessor(
        name="nlp-stream",
        processor=lambda chunk: pipeline.process(chunk),
        batch_size=32,
        max_concurrent=10,
    )
```

---

## Alternatives Considered

### Alternative 1: Hugging Face Transformers Only

**Description**: Use Hugging Face Transformers as the sole NLP engine.

**Pros**:
- Single dependency ecosystem
- Largest model collection
- Active research integration

**Cons**:
- Less production-ready than spaCy
- Steeper learning curve
- Higher memory usage
- Less type-safe interfaces

**Decision**: Rejected - spaCy provides better production readiness and type safety.

### Alternative 2: Custom Implementation

**Description**: Build a custom NLP engine from scratch.

**Pros**:
- Complete control over architecture
- No external dependencies
- Tailored to specific needs

**Cons**:
- Significant development effort
- Reinventing existing solutions
- Maintenance burden
- Lack of community support

**Decision**: Rejected - development effort and maintenance burden outweigh benefits.

### Alternative 3: LangChain as Core

**Description**: Use LangChain as the primary NLP orchestration layer.

**Pros**:
- Comprehensive LLM integration
- Rich ecosystem of tools
- Strong community

**Cons**:
- Overly complex for basic NLP tasks
- Rapid changes breaking existing code
- Abstraction layers obscuring performance
- Not designed for traditional NLP tasks

**Decision**: Rejected - LangChain is designed for LLM applications, not traditional NLP pipelines.

---

## Validation

### Success Criteria

1. **Performance**: Pipeline processes 10K+ documents/second on standard hardware
2. **Accuracy**: NER accuracy >90% F1 on standard benchmarks
3. **Reliability**: 99.9% uptime in production
4. **Type Safety**: 100% type coverage for public APIs
5. **Test Coverage**: >90% code coverage for core components

### Validation Plan

1. **Unit Tests**: Test individual components in isolation
2. **Integration Tests**: Test component interactions
3. **Performance Tests**: Benchmark against success criteria
4. **Load Tests**: Test under production-like load
5. **User Acceptance**: Validate with application teams

### Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Throughput | 10K docs/sec | Benchmark suite |
| Latency (p99) | <100ms | APM monitoring |
| Memory Usage | <2GB | Resource monitoring |
| Error Rate | <0.1% | Error tracking |
| Type Coverage | 100% | mypy strict mode |
| Test Coverage | >90% | pytest-cov |

---

## References

1. [NLP_TOOLKITS_SOTA.md](../research/NLP_TOOLKITS_SOTA.md) - State-of-the-art research on NLP toolkits
2. [ADR-002-tokenization.md](ADR-002-tokenization.md) - Tokenization strategy decision
3. [ADR-003-model-integration.md](ADR-003-model-integration.md) - ML model integration approach
4. spaCy Documentation: https://spacy.io/usage
5. Hugging Face Transformers: https://huggingface.co/docs/transformers
6. Phenotype Architecture Guidelines: /repos/AgilePlus/docs/

---

*This ADR was accepted on 2026-04-03 by the Phenotype Architecture Team.*
