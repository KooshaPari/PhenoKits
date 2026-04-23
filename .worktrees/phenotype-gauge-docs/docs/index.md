---
layout: home

hero:
  name: phenotype-gauge
  text: Benchmarking & xDD Testing
  tagline: Statistical benchmarking, property testing, and contract verification for Rust.
  actions:
    - theme: brand
      text: Get Started
      link: /getting-started
    - theme: alt
      text: API Reference
      link: /reference/api

features:
  - title: Property-Based Testing
    details: Powered by proptest and quickcheck to generate large input spaces and shrink failures to minimal reproductions.
  - title: Mutation Testing
    details: Verify that the suite is sensitive to real code changes by tracking killed mutants and survivors.
  - title: Contract Testing
    details: Encode domain invariants as explicit contracts that can be checked during test execution.
  - title: Statistical Benchmarking
    details: Built on criterion for statistically grounded measurements, regression checks, and CI-friendly output.
  - title: Spec Testing
    details: Map test cases directly to specification items so assertions stay tied to requirements.
  - title: Domain-First Layout
    details: Modules mirror xDD domain concepts such as contract, mutation, property, and spec.
---
