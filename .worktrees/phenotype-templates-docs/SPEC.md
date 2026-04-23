# Phenotype Templates Specification

> Template registry and management system

## Overview

Phenotype Templates provides a unified template registry for generating new projects with consistent structure, tooling, and best practices.

## Architecture

```
┌─────────────────────────────────────┐
│         Template Registry             │
│         (registry/index.json)         │
├─────────────────────────────────────┤
│           Template CLI                │
│     (list, generate, validate)        │
├─────────────────────────────────────┤
│         Template Engine               │
│       (Cookiecutter/Jinja2)           │
├─────────────────────────────────────┤
│         Template Sources              │
│    (templates/{language}/)            │
└─────────────────────────────────────┘
```

## Template Structure

Each template follows this structure:

```
templates/{language}/
├── cookiecutter.json          # Template variables
├── hooks/                     # Pre/post generation
│   ├── pre_gen_project.py
│   └── post_gen_project.py
└── {{cookiecutter.project_name}}/
    ├── README.md
    ├── src/
    ├── tests/
    └── .github/
```

## Template Variables

Common variables across all templates:

```json
{
  "project_name": "my-project",
  "description": "A new project",
  "author": "Your Name",
  "email": "your.email@example.com",
  "version": "0.1.0",
  "license": "MIT"
}
```

## Registry API

```typescript
interface TemplateRegistry {
  list(): Promise<Template[]>;
  get(name: string): Promise<Template>;
  generate(name: string, variables: object): Promise<string>;
  validate(name: string): Promise<boolean>;
}

interface Template {
  name: string;
  language: string;
  framework: string;
  status: "stable" | "beta" | "alpha";
  path: string;
  tags: string[];
}
```

## References

- [Cookiecutter](https://cookiecutter.readthedocs.io/)
- [Jinja2](https://jinja.palletsprojects.com/)
