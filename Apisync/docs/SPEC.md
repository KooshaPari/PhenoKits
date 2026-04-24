# SPEC: API Synchronization System

## Table of Contents

1. Overview
2. Architecture
3. OpenAPI Specification
4. Contract Testing
5. Version Management
6. Deployment

## Overview

API synchronization using OpenAPI specifications with contract testing.

## Architecture

```
OpenAPI Spec
├── Code Generation
├── Contract Testing
└── API Deployment
```

## OpenAPI Specification

```yaml
openapi: 3.1.0
info:
  title: Phenotype API
  version: 1.0.0
paths:
  /features:
    get:
      operationId: listFeatures
      responses:
        '200':
          description: Feature list
```

## Contract Testing

```yaml
contract-tests:
  - oasdiff breaking base.yaml head.yaml
  - schemathesis run --spec api.yaml
```

## Version Management

- URL path versioning (/v1/, /v2/)
- Sunset headers for deprecation
- 6-month deprecation period

## Deployment

```bash
# Generate code
openapi-generator generate -i api.yaml -g go

# Run tests
schemathesis run --spec api.yaml
```

---
*Specification Version: 1.0*
*Last Updated: 2026-04-05*
