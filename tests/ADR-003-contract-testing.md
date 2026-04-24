# ADR-003: Contract Testing Integration

## Status

**Accepted** — 2026-04-04

## Context

The Phenotype ecosystem has service boundaries that require stable APIs:

| Consumer | Provider | API Type |
|----------|----------|----------|
| phench | heliosCLI | REST API |
| heliosApp | heliosCLI | REST API |
| heliosCLI | External APIs | REST + GraphQL |
| src/ | Multiple services | REST |

Current challenges:
1. **Breaking Changes:** API changes in one service break consumers
2. **Late Discovery:** Integration issues found in staging/production
3. **Documentation Drift:** API docs don't match implementation
4. **Testing Gaps:** Consumers test against mocks that don't reflect reality

We need a strategy to:
- Detect breaking API changes before deployment
- Enable independent service deployment
- Document APIs through executable contracts
- Replace end-to-end integration tests with faster contract tests

Research findings (see SOTA.md):
- Contract testing prevents 60% of breaking API changes
- Pact is the industry standard for consumer-driven contracts
- Bi-directional contracts (OpenAPI + Pact) provide coverage validation

## Decision

We implement **bi-directional contract testing** using Pact:

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                    CONTRACT TESTING ARCHITECTURE                            │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│  ┌────────────────────────────────────────────────────────────────────────┐  │
│  │                      CONTRACT GENERATION                                │  │
│  │                                                                         │  │
│  │  Consumer (phench)              Provider (heliosCLI)                    │  │
│  │                                                                         │  │
│  │  ┌──────────────────┐          ┌──────────────────┐                      │  │
│  │  │ Consumer Tests   │          │ Provider Tests   │                      │  │
│  │  │                  │          │                  │                      │  │
│  │  │ mock_provider ───┼──────────┼──▶ verify()       │                      │  │
│  │  │      │          │          │       │          │                      │  │
│  │  │      ▼          │          │       ▼          │                      │  │
│  │  │  ┌──────────┐   │          │  ┌──────────┐    │                      │  │
│  │  │  │  Pact    │   │          │  │  Pact    │    │                      │  │
│  │  │  │  File    │───┼──────────┼──▶  File    │    │                      │  │
│  │  │  │ (.json)  │   │          │  │ (.json)  │    │                      │  │
│  │  │  └──────────┘   │          │  └──────────┘    │                      │  │
│  │  └──────────────────┘          └──────────────────┘                      │  │
│  │          │                              │                              │  │
│  │          └──────────────┬───────────────┘                              │  │
│  │                         │                                              │  │
│  │                         ▼                                              │  │
│  │  ┌────────────────────────────────────────────────────────────────┐   │  │
│  │  │                     PACT BROKER                                   │   │  │
│  │  │                                                                  │   │  │
│  │  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐           │   │  │
│  │  │  │ Contract     │  │ Version      │  │ Tags         │           │   │  │
│  │  │  │ Store        │  │ History      │  │ (dev/staging│           │   │  │
│  │  │  └──────────────┘  └──────────────┘  └──────────────┘           │   │  │
│  │  │                                                                  │   │  │
│  │  │  API:                                                            │   │  │
│  │  │  - publish-contracts                                             │   │  │
│  │  │  - verify-provider                                               │   │  │
│  │  │  - can-i-deploy (compatibility check)                            │   │  │
│  │  │  - webhook-notifications                                         │   │  │
│  │  │                                                                  │   │  │
│  │  └────────────────────────────────────────────────────────────────┘   │  │
│  │                                    │                                  │  │
│  │                                    ▼                                  │  │
│  │  ┌────────────────────────────────────────────────────────────────┐   │  │
│  │  │                      CI/CD INTEGRATION                          │   │  │
│  │  │                                                                  │   │  │
│  │  │  Consumer CI:                                                    │   │  │
│  │  │  1. Run consumer tests → generate pacts                         │   │  │
│  │  │  2. Publish pacts to broker (tag: pr-{branch})                   │   │  │
│  │  │  3. Trigger provider verification (webhook)                    │   │  │
│  │  │                                                                  │   │  │
│  │  │  Provider CI:                                                    │   │  │
│  │  │  1. Run provider verification against pacts                      │   │  │
│  │  │  2. Publish verification results                                 │   │  │
│  │  │  3. Check can-i-deploy before merge                              │   │  │
│  │  │                                                                  │   │  │
│  │  │  Deployment Gate:                                               │   │  │
│  │  │  - can-i-deploy --to-environment production                      │   │  │
│  │  │  - Blocks deployment if contracts not satisfied                  │   │  │
│  │  │                                                                  │   │  │
│  │  └────────────────────────────────────────────────────────────────┘   │  │
│  └────────────────────────────────────────────────────────────────────────┘  │
│                                                                               │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Bi-Directional Contract Flow

For internal services with OpenAPI specs:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│               BI-DIRECTIONAL CONTRACT VALIDATION                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────────────┐              ┌─────────────────────────┐        │
│  │   OpenAPI Spec          │              │   Consumer Pact         │        │
│  │   (heliosCLI)           │              │   (phench tests)        │        │
│  │                         │              │                         │        │
│  │  paths:                 │              │  {                      │        │
│  │    /users/{id}:         │              │    "interactions": [    │        │
│  │      get:               │              │      {                  │        │
│  │        responses:       │              │        "request": {     │        │
│  │          200:           │              │          "method": "GET"│        │
│  │            content:     │              │          "path": "/users/1"│       │
│  │              ...        │              │        },               │        │
│  │                         │              │        "response": {    │        │
│  │                         │              │          "status": 200   │        │
│  │                         │              │        }                │        │
│  │                         │              │      }                  │        │
│  │                         │              │    ]                    │        │
│  │                         │              │  }                      │        │
│  └───────────┬─────────────┘              └───────────┬─────────────┘        │
│              │                                      │                        │
│              │           ┌──────────────────┐      │                        │
│              └──────────▶│   Pact Broker    │◀─────┘                        │
│                          │   Validation     │                               │
│                          │                  │                               │
│                          │  Checks:         │                               │
│                          │  1. All consumer  │                               │
│                          │     interactions  │                               │
│                          │     covered by    │                               │
│                          │     OpenAPI spec    │                               │
│                          │  2. OpenAPI spec   │                               │
│                          │     valid per      │                               │
│                          │     consumer needs │                               │
│                          └──────────────────┘                               │
│                                                                              │
│  Result: Consumer tests validate OpenAPI spec is sufficient                   │
│          Provider tests validate OpenAPI spec is correct                      │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Contract Testing Levels

| Level | Description | When to Use | Example |
|-------|-------------|-------------|---------|
| L1 | Consumer unit tests with mock | Always | phench tests heliosCLI API |
| L2 | Provider verification against pacts | Required | heliosCI validates against phench pacts |
| L3 | Bi-directional (OpenAPI + Pact) | Internal APIs | heliosCLI provides OpenAPI, phench validates |
| L4 | E2E contract validation | Critical flows | Full integration with real services |

### Implementation Standards

#### Consumer Test Pattern (Python)

```python
# tests/contracts/test_helioscli_contracts.py
import pytest
from pact import Consumer, Provider

@pytest.fixture
def pact():
    return Consumer("phench").has_pact_with(
        Provider("helioscli"),
        pact_dir="./pacts",
        version="3.0.0"  # Pact spec version
    )

class TestHeliosCLIContracts:
    """Consumer contract tests for heliosCLI API."""
    
    def test_get_user_by_id(self, pact):
        """Contract: GET /users/{id} returns user details."""
        expected = {
            "id": "user-123",
            "email": "user@example.com",
            "role": "admin",
            "created_at": "2024-01-15T10:30:00Z"
        }
        
        (pact
         .given("user with id 'user-123' exists")
         .upon_receiving("a request for user by id")
         .with_request("GET", "/users/user-123")
         .will_respond_with(200, body=expected))
        
        with pact:
            # Actual HTTP call to mock provider
            result = helioscli_client.get_user("user-123")
            assert result["id"] == "user-123"
    
    def test_create_user(self, pact):
        """Contract: POST /users creates new user."""
        request_body = {
            "email": "new@example.com",
            "role": "user"
        }
        
        response_body = {
            "id": Like("user-new-id"),  # Match type, not value
            "email": "new@example.com",
            "role": "user",
            "created_at": Like("2024-01-15T10:30:00Z")
        }
        
        (pact
         .given("email 'new@example.com' is available")
         .upon_receiving("a request to create user")
         .with_request("POST", "/users", body=request_body)
         .will_respond_with(201, body=response_body))
        
        with pact:
            result = helioscli_client.create_user(
                email="new@example.com",
                role="user"
            )
            assert result["email"] == "new@example.com"
```

#### Provider Verification Pattern (Python)

```python
# tests/contracts/test_helioscli_provider.py
import pytest
from pact.verifier import Verifier

class TestHeliosCLIProvider:
    """Provider verification against consumer pacts."""
    
    @pytest.fixture
    def app(self):
        """Create test app with real dependencies."""
        return create_app(testing=True)
    
    def test_honours_pact_with_phench(self, app):
        """Verify heliosCLI satisfies phench contracts."""
        verifier = Verifier(
            provider="helioscli",
            provider_base_url="http://localhost:5000"
        )
        
        # Start test server
        with app.test_server() as server:
            output, return_code = verifier.verify_pacts(
                "./pacts/phench-helioscli.json",
                provider_states_setup_url=f"{server.base_url}/_pact/setup"
            )
        
        assert return_code == 0, f"Provider verification failed:\n{output}"
    
    def test_honours_all_consumer_pacts(self, app):
        """Verify against all consumers from Pact Broker."""
        verifier = Verifier(
            provider="helioscli",
            provider_base_url="http://localhost:5000"
        )
        
        with app.test_server() as server:
            output, return_code = verifier.verify_with_broker(
                broker_url=os.environ["PACT_BROKER_URL"],
                broker_token=os.environ["PACT_BROKER_TOKEN"],
                provider_states_setup_url=f"{server.base_url}/_pact/setup",
                publish_verification_results=True,
                provider_app_version=os.environ["GIT_COMMIT"]
            )
        
        assert return_code == 0
```

#### Provider State Setup

```python
# app/pact_provider_states.py
from flask import Blueprint, request, jsonify

pact_bp = Blueprint('pact', __name__)

@pact_bp.route('/_pact/setup', methods=['POST'])
def setup_provider_state():
    """Setup provider state for pact verification."""
    state = request.json.get('state')
    params = request.json.get('params', {})
    
    state_handlers = {
        'user with id {id} exists': _create_user,
        'email {email} is available': _clear_user_by_email,
        'no users exist': _clear_all_users,
    }
    
    handler = state_handlers.get(state)
    if handler:
        handler(**params)
    
    return jsonify({"status": "ok"})

def _create_user(id: str, **kwargs):
    """Create test user for provider state."""
    UserFactory.create(id=id, **kwargs)

def _clear_user_by_email(email: str):
    """Remove user by email if exists."""
    User.query.filter_by(email=email).delete()
    db.session.commit()

def _clear_all_users():
    """Remove all users."""
    User.query.delete()
    db.session.commit()
```

### CI/CD Integration

```yaml
# .github/workflows/contracts.yml
name: Contract Tests

on:
  push:
    branches: [main]
  pull_request:

jobs:
  consumer-contract-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Run consumer tests
        run: |
          pytest tests/contracts/ -v
      
      - name: Publish pacts to broker
        if: github.event_name == 'push'
        run: |
          pact-broker publish pacts/ \
            --consumer-app-version ${{ github.sha }} \
            --broker-base-url ${{ secrets.PACT_BROKER_URL }} \
            --broker-token ${{ secrets.PACT_BROKER_TOKEN }} \
            --tag ${{ github.ref_name }}

  provider-verification:
    runs-on: ubuntu-latest
    needs: consumer-contract-tests
    if: github.event_name == 'pull_request'
    steps:
      - uses: actions/checkout@v4
      
      - name: Verify provider
        run: |
          pytest tests/contracts/test_provider.py -v
        env:
          PACT_BROKER_URL: ${{ secrets.PACT_BROKER_URL }}
          PACT_BROKER_TOKEN: ${{ secrets.PACT_BROKER_TOKEN }}
      
      - name: Check can-i-deploy
        run: |
          pact-broker can-i-deploy \
            --pacticipant helioscli \
            --version ${{ github.sha }} \
            --to-environment production \
            --retry-while-unknown 6 \
            --retry-interval 10

  deploy-gate:
    runs-on: ubuntu-latest
    needs: [consumer-contract-tests, provider-verification]
    if: github.ref == 'refs/heads/main'
    steps:
      - name: Verify contracts before deploy
        run: |
          pact-broker can-i-deploy \
            --pacticipant helioscli \
            --version ${{ github.sha }} \
            --to-environment production
```

## Consequences

### Positive

1. **Early Detection:** Breaking changes caught in PR, not production
2. **Independent Deployment:** Services deploy without coordination
3. **Living Documentation:** Contracts are executable API docs
4. **Consumer Confidence:** Consumers test against real provider behavior
5. **Reduced E2E Tests:** Contract tests replace slow integration tests

### Negative

1. **Tooling Complexity:** Pact Broker infrastructure required
2. **Test Overhead:** Additional test layer to maintain
3. **Provider State Management:** Complex setup for verification
4. **Learning Curve:** Team must learn contract testing patterns

### Mitigations

| Risk | Mitigation |
|------|------------|
| Broker infrastructure | Start with Pact Broker SaaS (pactflow.io) |
| Test overhead | Generate contracts from existing API tests |
| Provider state | Use factory_boy for test data; document patterns |
| Learning curve | Training session + pair programming on first contracts |

## Rollout Plan

### Phase 1: Pilot (Week 1-2)

- [ ] Set up Pact Broker (SaaS)
- [ ] Implement contracts between phench ↔ heliosCLI
- [ ] Document patterns and run team training

### Phase 2: Critical Paths (Week 3-4)

- [ ] Add contracts for all internal service pairs
- [ ] Integrate with CI/CD (can-i-deploy gates)
- [ ] Remove redundant E2E tests

### Phase 3: External APIs (Month 2)

- [ ] Add contracts for external API consumers
- [ ] Implement bi-directional for services with OpenAPI
- [ ] Full deployment gate automation

### Success Criteria

| Metric | Target |
|--------|--------|
| Contract coverage | 100% of inter-service APIs |
| Verification time | < 5 minutes in CI |
| Breaking changes caught | > 90% before deployment |
| E2E test reduction | 50% replaced by contracts |

## Alternatives Considered

### Alternative 1: OpenAPI-First Only

Use OpenAPI specs without Pact verification.

**Rejected:**
- OpenAPI specs drift from implementation
- No consumer-driven validation
- Can't verify consumer expectations are met

### Alternative 2: Postman/Newman Tests

Collection-based API tests.

**Rejected:**
- Not consumer-driven
- Proprietary format
- No versioning/negotiation

### Alternative 3: End-to-End Only

Maintain comprehensive E2E test suite.

**Rejected:**
- Too slow for CI feedback
- Brittle, high maintenance
- Doesn't enable independent deployment

## References

- [Pact Documentation](https://docs.pact.io/)
- [Pact Broker](https://docs.pact.io/pact_broker)
- [Consumer-Driven Contracts](https://martinfowler.com/articles/consumerDrivenContracts.html)
- [SOTA.md — Contract Testing Research](./SOTA.md)

---

*ADR-003 — Contract Testing Integration*
