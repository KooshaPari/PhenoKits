# ADR-002: Tokenization Strategy

**Document ID:** PHENOTYPE_PHENOLANG_ADR-002  
**Status:** Accepted  
**Last Updated:** 2026-04-03  
**Author:** Phenotype Architecture Team  
**Supersedes:** N/A  
**Related:** [NLP_TOOLKITS_SOTA.md](../research/NLP_TOOLKITS_SOTA.md), [ADR-001-nlp-engine.md](ADR-001-nlp-engine.md), [ADR-003-model-integration.md](ADR-003-model-integration.md)

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

Tokenization is the foundational step in any NLP pipeline, converting raw text into discrete units that can be processed by downstream components. The choice of tokenization strategy significantly impacts:

- Model performance and accuracy
- Memory usage and computational efficiency
- Multilingual capabilities
- Handling of domain-specific terminology
- Compatibility with pre-trained models
- Streaming and real-time processing

PhenoLang requires a tokenization strategy that:

1. Supports multiple tokenization algorithms (BPE, WordPiece, Unigram, sentence-level)
2. Handles multilingual text seamlessly
3. Integrates with the chosen NLP engine (spaCy + Hugging Face per ADR-001)
4. Provides consistent interfaces across different tokenization backends
5. Supports both batch and streaming processing modes
6. Enables domain-specific customizations (technical terms, medical terminology, etc.)

### Current State

Tokenization capabilities are currently scattered across multiple packages with inconsistent interfaces:

```
┌─────────────────────────────────────────────────────────────────────┐
│              Current Tokenization Fragmentation                     │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  pheno-vector/embeddings/                                           │
│  └── Uses Hugging Face tokenizers (implicit)                       │
│                                                                     │
│  pheno-tools/shared/                                                │
│  └── Ad-hoc text splitting utilities                               │
│                                                                     │
│  pheno-core/stream.py                                               │
│  └── Stream chunking (not linguistic tokenization)                 │
│                                                                     │
│  PROBLEM: No unified tokenization interface, inconsistent behavior  │
│  across packages, no support for domain-specific tokenization.      │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### Tokenization Requirements

| Requirement | Priority | Description |
|-------------|----------|-------------|
| Multi-algorithm support | High | Support BPE, WordPiece, Unigram, sentence-level |
| Multilingual | High | Handle 20+ languages with appropriate tokenizers |
| Performance | High | Process 100K+ tokens/second |
| Type safety | Medium | Strong typing for token data structures |
| Streaming | Medium | Support streaming tokenization |
| Domain-specific | Medium | Allow custom tokenization rules |
| Reversibility | Low | Ability to reconstruct original text |
| Subword awareness | Low | Expose subword structure for downstream tasks |

### Constraints

1. **Model Compatibility**: Must produce tokens compatible with pre-trained models (spaCy, Hugging Face)
2. **Memory Efficiency**: Tokenizer models must fit within reasonable memory constraints (<500MB)
3. **Thread Safety**: Must support concurrent tokenization from multiple threads
4. **Serialization**: Tokenizer state must be serializable for caching and distribution

---

## Decision

### Selected Approach: Multi-Backend Tokenization with Unified Interface

We will implement a **unified tokenization interface** that abstracts over multiple tokenization backends:

1. **SentencePiece** as the primary subword tokenizer (BPE and Unigram models)
2. **spaCy** for sentence-level and linguistic tokenization
3. **Tiktoken** for OpenAI model compatibility
4. **Custom tokenizers** for domain-specific requirements

### Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                    Tokenization Architecture                        │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                 TokenizerRegistry                           │   │
│  │  - Register tokenizers by name                             │   │
│  │  - Resolve tokenizer by name/language/task                 │   │
│  │  - Manage tokenizer lifecycle                              │   │
│  └───────────────────────────┬─────────────────────────────────┘   │
│                              │                                      │
│  ┌───────────────────────────▼─────────────────────────────────┐   │
│  │              TokenizerProtocol (Interface)                  │   │
│  │  - tokenize(text: str) -> List[Token]                      │   │
│  │  - tokenize_batch(texts: List[str]) -> List[List[Token]]   │   │
│  │  - encode(text: str) -> List[int]                          │   │
│  │  - decode(ids: List[int]) -> str                           │   │
│  │  - save(path: str) / load(path: str)                       │   │
│  └───────┬───────────────┬───────────────┬─────────────────────┘   │
│          │               │               │                          │
│  ┌───────▼──────┐ ┌──────▼──────┐ ┌──────▼──────┐                  │
│  │ SentencePiece│ │   spaCy     │ │  Tiktoken   │                  │
│  │ Tokenizer    │ │  Tokenizer  │ │  Tokenizer  │                  │
│  │              │ │             │ │             │                  │
│  │ - BPE        │ │ - Sentence  │ │ - OpenAI    │                  │
│  │ - Unigram    │ │ - Word      │ │ - Cl100k    │                  │
│  │ - Language   │ │ - Linguistic│ │ - P50k      │                  │
│  │   agnostic   │ │ - POS-aware │ │ - R50k      │                  │
│  └──────────────┘ └─────────────┘ └─────────────┘                  │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │              Domain-Specific Tokenizers                     │   │
│  │  - Medical terminology                                     │   │
│  │  - Code/tokenization                                       │   │
│  │  - Technical jargon                                        │   │
│  │  - Custom vocabulary                                       │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### Token Data Model

```python
"""Tokenization data models."""

from typing import List, Optional, Dict, Any
from dataclasses import dataclass, field
from enum import Enum


class TokenType(Enum):
    """Types of tokens."""
    WORD = "word"
    PUNCTUATION = "punctuation"
    WHITESPACE = "whitespace"
    SUBWORD = "subword"
    SENTENCE = "sentence"
    SPECIAL = "special"  # [CLS], [SEP], [PAD], etc.


@dataclass
class Token:
    """Represents a single token with metadata."""
    text: str
    token_type: TokenType
    start: int  # Start position in original text
    end: int    # End position in original text
    id: Optional[int] = None  # Token ID (for subword tokenizers)
    pos_tag: Optional[str] = None  # Part-of-speech tag
    lemma: Optional[str] = None  # Lemmatized form
    is_oov: bool = False  # Out-of-vocabulary flag
    metadata: Dict[str, Any] = field(default_factory=dict)

    @property
    def length(self) -> int:
        """Token length in characters."""
        return self.end - self.start

    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary."""
        return {
            "text": self.text,
            "type": self.token_type.value,
            "start": self.start,
            "end": self.end,
            "id": self.id,
            "pos_tag": self.pos_tag,
            "lemma": self.lemma,
            "is_oov": self.is_oov,
            "metadata": self.metadata,
        }

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "Token":
        """Create Token from dictionary."""
        return cls(
            text=data["text"],
            token_type=TokenType(data["type"]),
            start=data["start"],
            end=data["end"],
            id=data.get("id"),
            pos_tag=data.get("pos_tag"),
            lemma=data.get("lemma"),
            is_oov=data.get("is_oov", False),
            metadata=data.get("metadata", {}),
        )


@dataclass
class TokenizationResult:
    """Result of tokenization."""
    tokens: List[Token]
    tokenizer_name: str
    original_text: str
    num_tokens: int
    num_oov: int = 0
    processing_time: float = 0.0
    metadata: Dict[str, Any] = field(default_factory=dict)

    @property
    def token_texts(self) -> List[str]:
        """Extract token texts."""
        return [t.text for t in self.tokens]

    @property
    def token_ids(self) -> List[int]:
        """Extract token IDs."""
        return [t.id for t in self.tokens if t.id is not None]

    def reconstruct(self) -> str:
        """Reconstruct original text from tokens."""
        return self.original_text

    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary."""
        return {
            "tokens": [t.to_dict() for t in self.tokens],
            "tokenizer": self.tokenizer_name,
            "original_text": self.original_text,
            "num_tokens": self.num_tokens,
            "num_oov": self.num_oov,
            "processing_time": self.processing_time,
            "metadata": self.metadata,
        }
```

### Tokenizer Interface

```python
"""Tokenizer protocol and implementations."""

from typing import Protocol, List, Optional
from abc import ABC, abstractmethod

from pheno_nlp.tokenization.models import Token, TokenizationResult


class Tokenizer(Protocol):
    """Protocol for tokenizer implementations."""

    name: str
    version: str

    def tokenize(self, text: str) -> TokenizationResult:
        """Tokenize text into tokens."""
        ...

    def tokenize_batch(self, texts: List[str]) -> List[TokenizationResult]:
        """Tokenize multiple texts."""
        ...

    def encode(self, text: str) -> List[int]:
        """Encode text to token IDs."""
        ...

    def decode(self, ids: List[int]) -> str:
        """Decode token IDs to text."""
        ...

    def save(self, path: str) -> None:
        """Save tokenizer to disk."""
        ...

    @classmethod
    def load(cls, path: str) -> "Tokenizer":
        """Load tokenizer from disk."""
        ...


class SentencePieceTokenizer:
    """SentencePiece tokenizer implementation."""

    name = "sentencepiece"
    version = "1.0.0"

    def __init__(self, model_path: Optional[str] = None, vocab_size: int = 30000):
        import sentencepiece as spm
        self._spm = spm.SentencePieceProcessor()
        self._vocab_size = vocab_size

        if model_path:
            self._spm.Load(model_path)
        else:
            # Use default unigram model
            self._spm.LoadFromSerializedProto(self._get_default_model())

    def tokenize(self, text: str) -> TokenizationResult:
        """Tokenize text using SentencePiece."""
        import time
        start = time.perf_counter()

        pieces = self._spm.EncodeAsPieces(text)
        ids = self._spm.EncodeAsIds(text)

        tokens = []
        pos = 0
        for piece, token_id in zip(pieces, ids):
            # Find token position in original text
            start_pos = text.find(piece, pos)
            if start_pos == -1:
                start_pos = pos
            end_pos = start_pos + len(piece.lstrip("▁"))

            token_type = self._determine_token_type(piece)
            is_oov = token_id == self._spm.unk_id()

            tokens.append(Token(
                text=piece.lstrip("▁"),
                token_type=token_type,
                start=start_pos,
                end=end_pos,
                id=token_id,
                is_oov=is_oov,
            ))
            pos = end_pos

        processing_time = time.perf_counter() - start
        num_oov = sum(1 for t in tokens if t.is_oov)

        return TokenizationResult(
            tokens=tokens,
            tokenizer_name=self.name,
            original_text=text,
            num_tokens=len(tokens),
            num_oov=num_oov,
            processing_time=processing_time,
        )

    def tokenize_batch(self, texts: List[str]) -> List[TokenizationResult]:
        """Tokenize multiple texts."""
        return [self.tokenize(text) for text in texts]

    def encode(self, text: str) -> List[int]:
        """Encode text to token IDs."""
        return self._spm.EncodeAsIds(text)

    def decode(self, ids: List[int]) -> str:
        """Decode token IDs to text."""
        return self._spm.DecodeIds(ids)

    def save(self, path: str) -> None:
        """Save tokenizer model."""
        self._spm.Save(path)

    @classmethod
    def load(cls, path: str) -> "SentencePieceTokenizer":
        """Load tokenizer from disk."""
        instance = cls.__new__(cls)
        import sentencepiece as spm
        instance._spm = spm.SentencePieceProcessor()
        instance._spm.Load(path)
        return instance

    def _determine_token_type(self, piece: str) -> "TokenType":
        """Determine token type from piece."""
        from pheno_nlp.tokenization.models import TokenType
        if piece.startswith("▁"):
            return TokenType.WORD
        elif piece.strip() in ".,!?;:()[]{}\"'":
            return TokenType.PUNCTUATION
        elif piece.strip() == "":
            return TokenType.WHITESPACE
        else:
            return TokenType.SUBWORD

    def _get_default_model(self) -> bytes:
        """Get default model bytes."""
        # In practice, load from bundled model file
        raise NotImplementedError("Default model not implemented")
```

### Tokenizer Registry

```python
"""Tokenizer registry for managing multiple tokenizers."""

from typing import Dict, Type, Optional, List
from pheno_nlp.tokenization.base import Tokenizer


class TokenizerRegistry:
    """Registry for tokenizer implementations."""

    def __init__(self):
        self._tokenizers: Dict[str, Tokenizer] = {}
        self._tokenizer_classes: Dict[str, Type[Tokenizer]] = {}

    def register(self, name: str, tokenizer: Tokenizer) -> None:
        """Register a tokenizer instance."""
        self._tokenizers[name] = tokenizer

    def register_class(self, name: str, tokenizer_class: Type[Tokenizer]) -> None:
        """Register a tokenizer class for lazy instantiation."""
        self._tokenizer_classes[name] = tokenizer_class

    def get(self, name: str) -> Tokenizer:
        """Get a tokenizer by name."""
        if name in self._tokenizers:
            return self._tokenizers[name]

        if name in self._tokenizer_classes:
            tokenizer = self._tokenizer_classes[name]()
            self._tokenizers[name] = tokenizer
            return tokenizer

        raise KeyError(f"Tokenizer not found: {name}")

    def list_tokenizers(self) -> List[str]:
        """List all registered tokenizers."""
        return list(self._tokenizers.keys()) + list(self._tokenizer_classes.keys())

    def get_or_create(self, name: str, **kwargs) -> Tokenizer:
        """Get existing tokenizer or create new one."""
        try:
            return self.get(name)
        except KeyError:
            if name in self._tokenizer_classes:
                tokenizer = self._tokenizer_classes[name](**kwargs)
                self._tokenizers[name] = tokenizer
                return tokenizer
            raise

    def clear(self) -> None:
        """Clear all registered tokenizers."""
        self._tokenizers.clear()


# Global registry instance
registry = TokenizerRegistry()


def register_default_tokenizers():
    """Register default tokenizers."""
    from pheno_nlp.tokenization.sentencepiece import SentencePieceTokenizer
    from pheno_nlp.tokenization.spacy import SpaCyTokenizer
    from pheno_nlp.tokenization.tiktoken import TiktokenTokenizer

    registry.register_class("sentencepiece", SentencePieceTokenizer)
    registry.register_class("spacy", SpaCyTokenizer)
    registry.register_class("tiktoken", TiktokenTokenizer)
```

---

## Consequences

### Positive Consequences

1. **Unified Interface**: All tokenization operations use a consistent interface, making it easy to swap tokenizers without changing downstream code.

2. **Multi-Backend Support**: Support for multiple tokenization backends enables optimal tokenizer selection based on use case (performance vs. accuracy vs. compatibility).

3. **Type Safety**: Strong typing for token data structures enables compile-time error detection and better IDE support.

4. **Extensibility**: The registry pattern allows easy addition of new tokenizers without modifying existing code.

5. **Domain-Specific Support**: Custom tokenizers can be implemented for domain-specific requirements (medical, legal, technical).

6. **Performance Optimization**: Different tokenizers can be selected based on performance requirements (Tiktoken for speed, SentencePiece for accuracy).

7. **Model Compatibility**: Tokenizers are compatible with pre-trained models (spaCy, Hugging Face, OpenAI), enabling seamless integration.

8. **Streaming Support**: Tokenization can be performed in streaming mode for real-time processing.

### Negative Consequences

1. **Complexity**: Multiple tokenizer implementations increase code complexity and maintenance burden.

2. **Memory Overhead**: Loading multiple tokenizer models can consume significant memory.

3. **Training Overhead**: Training custom SentencePiece models requires computational resources and time.

4. **Version Management**: Keeping tokenizer models in sync with upstream models requires ongoing maintenance.

5. **Testing Burden**: Each tokenizer implementation requires comprehensive testing across multiple languages and edge cases.

6. **Documentation Overhead**: Multiple tokenizers require comprehensive documentation for users to make informed choices.

### Mitigation Strategies

| Risk | Mitigation |
|------|-----------|
| Complexity | Clear interfaces, comprehensive tests, documentation |
| Memory overhead | Lazy loading, model sharing, memory limits |
| Training overhead | Pre-trained models, automated training pipelines |
| Version management | Automated model updates, version pinning |
| Testing burden | Property-based testing, benchmark suites |
| Documentation overhead | Auto-generated docs, examples, tutorials |

---

## Implementation Details

### Tokenization Pipeline Integration

```python
"""Integration with NLP pipeline."""

from pheno_nlp.pipeline import NLPipeline
from pheno_nlp.tokenization.registry import registry
from pheno_nlp.tokenization.models import TokenizationResult


class TokenizationComponent:
    """Tokenization component for NLP pipeline."""

    def __init__(self, tokenizer_name: str = "sentencepiece"):
        self.tokenizer = registry.get(tokenizer_name)
        self.tokenizer_name = tokenizer_name

    def process(self, text: str) -> TokenizationResult:
        """Process text through tokenizer."""
        return self.tokenizer.tokenize(text)

    def batch_process(self, texts: List[str]) -> List[TokenizationResult]:
        """Process multiple texts."""
        return self.tokenizer.tokenize_batch(texts)


# Usage in pipeline
def create_tokenization_pipeline(tokenizer_name: str = "sentencepiece") -> NLPipeline:
    """Create a tokenization pipeline."""
    pipeline = NLPipeline(name="tokenization")
    pipeline.add_component(TokenizationComponent(tokenizer_name))
    return pipeline
```

### Domain-Specific Tokenization

```python
"""Domain-specific tokenization extensions."""

from typing import List, Set
from pheno_nlp.tokenization.base import Tokenizer
from pheno_nlp.tokenization.models import Token, TokenizationResult, TokenType


class MedicalTokenizer:
    """Tokenizer for medical text with domain-specific vocabulary."""

    name = "medical"
    version = "1.0.0"

    def __init__(self, base_tokenizer: Tokenizer, medical_terms: Set[str]):
        self.base_tokenizer = base_tokenizer
        self.medical_terms = medical_terms

    def tokenize(self, text: str) -> TokenizationResult:
        """Tokenize medical text."""
        result = self.base_tokenizer.tokenize(text)

        # Merge medical terms
        tokens = self._merge_medical_terms(result.tokens, text)

        return TokenizationResult(
            tokens=tokens,
            tokenizer_name=self.name,
            original_text=text,
            num_tokens=len(tokens),
            num_oov=sum(1 for t in tokens if t.is_oov),
            processing_time=result.processing_time,
            metadata={"domain": "medical"},
        )

    def _merge_medical_terms(self, tokens: List[Token], text: str) -> List[Token]:
        """Merge tokens that form medical terms."""
        merged = []
        i = 0
        while i < len(tokens):
            # Check for multi-word medical terms
            term = self._find_medical_term(tokens[i:], text)
            if term:
                merged.append(self._create_merged_token(term, tokens, i))
                i += len(term.split())
            else:
                merged.append(tokens[i])
                i += 1
        return merged

    def _find_medical_term(self, tokens: List[Token], text: str) -> Optional[str]:
        """Find medical term starting from current position."""
        for length in range(min(5, len(tokens)), 0, -1):
            term = " ".join(t.text for t in tokens[:length])
            if term.lower() in self.medical_terms:
                return term
        return None

    def _create_merged_token(self, term: str, tokens: List[Token], start_idx: int) -> Token:
        """Create merged token for medical term."""
        return Token(
            text=term,
            token_type=TokenType.WORD,
            start=tokens[start_idx].start,
            end=tokens[start_idx + term.split() - 1].end,
            metadata={"domain": "medical", "term": term.lower()},
        )
```

---

## Alternatives Considered

### Alternative 1: Single Tokenizer (SentencePiece Only)

**Description**: Use only SentencePiece for all tokenization needs.

**Pros**:
- Simpler architecture
- Single dependency to manage
- Consistent behavior

**Cons**:
- No OpenAI model compatibility
- Less optimal for sentence-level tasks
- No linguistic tokenization (POS, lemmas)

**Decision**: Rejected - need multiple tokenizers for different use cases.

### Alternative 2: Hugging Face Tokenizers Only

**Description**: Use Hugging Face tokenizers library exclusively.

**Pros**:
- Comprehensive tokenizer support
- Good performance
- Active development

**Cons**:
- Less type-safe interfaces
- No spaCy integration
- Limited linguistic tokenization

**Decision**: Rejected - need spaCy integration for linguistic tokenization.

### Alternative 3: Custom Tokenizer from Scratch

**Description**: Build a custom tokenizer implementation.

**Pros**:
- Complete control
- No external dependencies
- Tailored to specific needs

**Cons**:
- Significant development effort
- Reinventing existing solutions
- Maintenance burden
- Performance optimization challenges

**Decision**: Rejected - existing solutions are mature and well-optimized.

---

## Validation

### Success Criteria

1. **Performance**: Tokenize 100K+ tokens/second on standard hardware
2. **Accuracy**: Tokenization matches reference implementations (spaCy, Hugging Face)
3. **Multilingual**: Support 20+ languages with appropriate tokenizers
4. **Type Safety**: 100% type coverage for token data structures
5. **Test Coverage**: >90% code coverage for tokenization components

### Validation Plan

1. **Unit Tests**: Test individual tokenizers in isolation
2. **Integration Tests**: Test tokenizers with pipeline components
3. **Performance Tests**: Benchmark against success criteria
4. **Multilingual Tests**: Test across supported languages
5. **Edge Case Tests**: Test unusual text patterns, Unicode, etc.

### Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Throughput | 100K tokens/sec | Benchmark suite |
| Latency (p99) | <10ms per 1K tokens | Benchmark suite |
| Memory Usage | <500MB per tokenizer | Resource monitoring |
| Type Coverage | 100% | mypy strict mode |
| Test Coverage | >90% | pytest-cov |
| Language Support | 20+ languages | Multilingual test suite |

---

## References

1. [NLP_TOOLKITS_SOTA.md](../research/NLP_TOOLKITS_SOTA.md) - State-of-the-art research on tokenization
2. [ADR-001-nlp-engine.md](ADR-001-nlp-engine.md) - NLP engine selection decision
3. [ADR-003-model-integration.md](ADR-003-model-integration.md) - ML model integration approach
4. SentencePiece Documentation: https://github.com/google/sentencepiece
5. Hugging Face Tokenizers: https://huggingface.co/docs/tokenizers
6. Tiktoken: https://github.com/openai/tiktoken
7. spaCy Tokenization: https://spacy.io/usage/linguistic-features#tokenization

---

*This ADR was accepted on 2026-04-03 by the Phenotype Architecture Team.*
