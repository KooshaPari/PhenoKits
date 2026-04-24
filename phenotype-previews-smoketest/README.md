# phenotype-previews-smoketest

Smoke testing and preview validation suite for Phenotype documentation sites and deployment previews. Automated browser testing via Playwright for verifying documentation builds, feature previews, and live deployments.

## Overview

**phenotype-previews-smoketest** provides comprehensive smoke testing for Phenotype documentation sites and preview deployments. It uses Playwright for browser automation to validate:

- Documentation site rendering and navigation
- Feature preview functionality
- Deployment health checks
- Link validity
- Component rendering and interaction
- Accessibility basics

**Core Mission**: Catch documentation and deployment issues early through automated smoke testing of preview environments before promoting to production.

## Technology Stack

- **Test Framework**: Playwright (cross-browser automation)
- **Frontend**: Next.js 14+ (App Router)
- **Language**: TypeScript (strict mode)
- **Test Runner**: Playwright test runner
- **CI Integration**: GitHub Actions, Woodpecker
- **Browser Coverage**: Chromium, Firefox, WebKit

## Key Features

- **Automated Smoke Tests**: Playwright-based tests for all preview sites
- **Cross-Browser Testing**: Chrome, Firefox, Safari coverage
- **Link Validation**: Automated link checking across documentation
- **Component Testing**: Interactive component validation
- **Accessibility Checks**: Basic a11y compliance verification
- **Performance Metrics**: Collect Lighthouse scores and Core Web Vitals
- **Parallel Execution**: Fast test runs via Playwright's parallel mode
- **Screenshot Diffs**: Visual regression detection via screenshots
- **CI Integration**: Built-in GitHub Actions and Woodpecker support

## Quick Start

```bash
# Clone and explore
git clone <repo-url>
cd phenotype-previews-smoketest

# Review governance and architecture
cat CLAUDE.md          # Project governance
cat AGENTS.md          # Agent operating contract

# Install dependencies
npm install

# Run tests locally against staging preview
npm run test:staging

# Run tests against production
npm run test:production

# Run with UI for debugging
npm run test:ui

# Generate test report
npm run test:report
```

## Project Structure

```
phenotype-previews-smoketest/
├── app/                       # Next.js application (test dashboard)
│   ├── page.tsx               # Test results dashboard
│   ├── layout.tsx             # App layout
│   └── api/
│       └── results/           # Test results API endpoint
├── tests/
│   ├── smoke/
│   │   ├── docs-navigation.spec.ts      # Navigation smoke tests
│   │   ├── link-validation.spec.ts      # Link validity tests
│   │   ├── components.spec.ts           # Component interaction tests
│   │   └── accessibility.spec.ts        # a11y checks
│   ├── e2e/
│   │   ├── feature-previews.spec.ts     # Feature preview validation
│   │   └── user-journeys.spec.ts        # User path testing
│   ├── fixtures/
│   │   ├── pages.ts           # Common page objects
│   │   └── data.ts            # Test data fixtures
│   └── config/
│       ├── environments.ts    # Environment configuration
│       └── browsers.ts        # Browser configuration
├── pages/
│   ├── deployment-status.tsx  # Real-time deployment status
│   └── test-results.tsx       # Test execution results
├── utils/
│   ├── reporter.ts            # Custom test reporter
│   ├── screenshot-diff.ts     # Visual regression utilities
│   └── metrics.ts             # Performance metrics collection
├── reports/
│   └── index.html             # Test report templates
├── playwright.config.ts       # Playwright configuration
├── .github/workflows/
│   ├── smoke-test-staging.yml # Staging preview tests
│   └── smoke-test-prod.yml    # Production tests
├── .woodpecker.yml            # Woodpecker CI config
├── package.json
├── tsconfig.json
├── next.config.js
├── docs/
│   ├── WRITING_TESTS.md       # Guide to writing new tests
│   ├── DEBUGGING.md           # Debugging failed tests
│   └── CI_INTEGRATION.md      # CI setup and troubleshooting
└── README.md
```

## Test Types

### Smoke Tests (Fast, Core Checks)
- ✓ Site loads and renders
- ✓ Navigation works
- ✓ Main pages render without errors
- ✓ Links are valid
- ✓ Critical components work

### E2E Tests (Longer, Feature-Focused)
- ✓ Feature previews function correctly
- ✓ User journeys complete successfully
- ✓ Forms submit correctly
- ✓ API integrations work
- ✓ Real user workflows pass

### Visual Tests
- ✓ Screenshot comparisons
- ✓ Layout regression detection
- ✓ CSS rendering validation
- ✓ Responsive design checks

## Running Tests

```bash
# All tests in parallel
npm run test

# Smoke tests only (fast)
npm run test:smoke

# E2E tests only
npm run test:e2e

# Single test file
npm run test tests/smoke/link-validation.spec.ts

# Debug mode (shows browser)
npm run test:debug

# Generate HTML report
npm run test:report
open test-results/index.html
```

## CI/CD Integration

Tests run automatically on:
- Pull requests to documentation repos
- Deployment preview creation
- Staging deployments
- Production deployments

## Related Phenotype Projects

- **phenodocs**: Main documentation site (primary test target)
- **PhenoDevOps**: CI/CD orchestration (runs these tests)
- **cloud**: Multi-tenant platform (deployment validation)
- **Tracera**: Observability (test execution telemetry)
