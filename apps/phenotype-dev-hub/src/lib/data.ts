export const collections = {
  sidekick: {
    name: 'Sidekick',
    slug: 'sidekick',
    tagline: 'AI-powered agent framework for autonomous task execution',
    accent: 'sidekick',
    icon: '🤖',
    description: 'Sidekick is a modern agent framework enabling autonomous task execution, tool integration, and multi-step reasoning. Built for extensibility and production deployment.',
    members: [
      { name: 'Core Agent', description: 'Orchestrator for multi-step task execution' },
      { name: 'Tool Registry', description: 'Plugin system for integrating external tools' },
      { name: 'Memory & State', description: 'Persistent context and conversation history' },
      { name: 'Model Routing', description: 'Multi-model inference with cost optimization' }
    ],
    links: {
      github: 'https://github.com/kooshapari/sidekick',
      docs: 'https://sidekick.phenotype.dev'
    }
  },
  eidolon: {
    name: 'Eidolon',
    slug: 'eidolon',
    tagline: 'Semantic search and RAG infrastructure',
    accent: 'eidolon',
    icon: '🔍',
    description: 'Eidolon provides enterprise-grade semantic search, retrieval-augmented generation (RAG), and vector database infrastructure for AI applications.',
    members: [
      { name: 'Vector Storage', description: 'HNSW-based vector indexing and retrieval' },
      { name: 'Embedding Pipeline', description: 'Document processing and embedding generation' },
      { name: 'RAG Engine', description: 'Context retrieval for LLM augmentation' },
      { name: 'Semantic Router', description: 'Intent classification and routing' }
    ],
    links: {
      github: 'https://github.com/kooshapari/eidolon',
      docs: 'https://eidolon.phenotype.dev'
    }
  },
  paginary: {
    name: 'Paginary',
    slug: 'paginary',
    tagline: 'Progressive pagination and infinite scroll toolkit',
    accent: 'paginary',
    icon: '📖',
    description: 'Paginary is a comprehensive toolkit for building scalable pagination systems, infinite scroll UIs, and cursor-based navigation patterns.',
    members: [
      { name: 'Cursor Pagination', description: 'Stateless pagination with cursor-based traversal' },
      { name: 'Infinite Scroll', description: 'Progressive loading and intersection observer integration' },
      { name: 'Range Queries', description: 'Efficient slicing and range-based data fetching' },
      { name: 'Caching Layer', description: 'Smart cache invalidation and prefetching' }
    ],
    links: {
      github: 'https://github.com/kooshapari/paginary',
      docs: 'https://paginary.phenotype.dev'
    }
  },
  observably: {
    name: 'Observably',
    slug: 'observably',
    tagline: 'Observability and distributed tracing platform',
    accent: 'observably',
    icon: '📊',
    description: 'Observably is a comprehensive observability platform for distributed systems, providing metrics, traces, logs, and real-time dashboards.',
    members: [
      { name: 'Metrics Collector', description: 'Prometheus-compatible metrics scraping and aggregation' },
      { name: 'Trace Ingestor', description: 'OpenTelemetry-compatible distributed tracing' },
      { name: 'Log Aggregation', description: 'Structured logging with full-text search' },
      { name: 'Alert Engine', description: 'Rule-based alerting and incident management' }
    ],
    links: {
      github: 'https://github.com/kooshapari/observably',
      docs: 'https://observably.phenotype.dev'
    }
  },
  stashly: {
    name: 'Stashly',
    slug: 'stashly',
    tagline: 'Decentralized storage and content addressing',
    accent: 'stashly',
    icon: '💾',
    description: 'Stashly provides decentralized storage primitives with content-addressed retrieval, IPFS integration, and distributed ledger synchronization.',
    members: [
      { name: 'Content Addressing', description: 'Content-hash based storage and retrieval' },
      { name: 'IPFS Gateway', description: 'IPFS integration and pinning service' },
      { name: 'Sync Protocol', description: 'Peer-to-peer synchronization and replication' },
      { name: 'Encryption Layer', description: 'End-to-end encryption for stored content' }
    ],
    links: {
      github: 'https://github.com/kooshapari/stashly',
      docs: 'https://stashly.phenotype.dev'
    }
  }
};

export const products = {
  focalpoint: {
    name: 'FocalPoint',
    slug: 'focalpoint',
    description: 'Production-grade AI development platform with multi-model support, fine-tuning, and deployment infrastructure',
    accent: 'focalpoint',
    icon: '🎯',
    status: 'stable',
    tagline: 'AI development platform',
    details: {
      overview: 'FocalPoint is a comprehensive AI development platform enabling teams to build, train, deploy, and monitor AI applications at scale.',
      features: [
        'Multi-model inference with automatic routing',
        'Fine-tuning and transfer learning pipelines',
        'Production-grade API and SDKs',
        'Real-time monitoring and observability',
        'Team collaboration and version control'
      ]
    },
    links: {
      website: 'https://focalpoint.app',
      github: 'https://github.com/kooshapari/focalpoint',
      docs: 'https://docs.focalpoint.app'
    }
  },
  agileplus: {
    name: 'AgilePlus',
    slug: 'agileplus',
    description: 'Work management and agile planning platform with integrated observability and team collaboration',
    accent: 'agileplus',
    icon: '⚡',
    status: 'stable',
    tagline: 'Work management platform',
    details: {
      overview: 'AgilePlus is a modern work management system combining agile planning, team collaboration, and deep integration with development workflows.',
      features: [
        'Kanban, Scrum, and custom workflows',
        'Real-time collaboration and notifications',
        'Git integration and CI/CD pipeline tracking',
        'Time tracking and capacity planning',
        'Custom dashboards and reporting'
      ]
    },
    links: {
      website: 'https://agileplus.phenotype.dev',
      github: 'https://github.com/kooshapari/AgilePlus',
      docs: 'https://docs.agileplus.phenotype.dev'
    }
  },
  tracera: {
    name: 'Tracera',
    slug: 'tracera',
    description: 'Distributed tracing and performance analysis system for complex microservice architectures',
    accent: 'tracera',
    icon: '🔗',
    status: 'beta',
    tagline: 'Distributed tracing system',
    details: {
      overview: 'Tracera provides end-to-end distributed tracing, performance profiling, and dependency mapping for modern microservices.',
      features: [
        'Automatic service discovery and dependency mapping',
        'Low-overhead span collection and analysis',
        'Custom instrumentation SDK',
        'Performance anomaly detection',
        'Multi-tenant trace isolation'
      ]
    },
    links: {
      website: 'https://tracera.phenotype.dev',
      github: 'https://github.com/kooshapari/Tracera',
      docs: 'https://docs.tracera.phenotype.dev'
    }
  },
  hwledger: {
    name: 'hwLedger',
    slug: 'hwledger',
    description: 'Blockchain-based ledger system with cryptographic verification and distributed consensus',
    accent: 'hwledger',
    icon: '📜',
    status: 'alpha',
    tagline: 'Distributed ledger system',
    details: {
      overview: 'hwLedger is a lightweight distributed ledger providing cryptographic verification, append-only logs, and consensus mechanisms.',
      features: [
        'SHA-256 hash-chain verification',
        'Byzantine fault-tolerant consensus',
        'Smart contract execution',
        'Event sourcing capabilities',
        'Light client support'
      ]
    },
    links: {
      website: 'https://hwledger.phenotype.dev',
      github: 'https://github.com/kooshapari/hwLedger',
      docs: 'https://docs.hwledger.phenotype.dev'
    }
  }
};
