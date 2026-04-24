# PRD.md — tooling/browser-agent-mcp

## Overview

This PRD describes **browser-agent-mcp**, an MCP (Model Context Protocol) server that exposes browser automation capabilities to AI agents and Claude Desktop. It provides a standardized tool interface for navigating, interacting with, and extracting data from web pages.

---

## 1. Users & Use Cases

### Primary Users

| User | Role | Goals |
|------|------|-------|
| AI Agents | Autonomous web scrapers | Navigate pages, extract data, fill forms |
| Claude Desktop | Interactive web browser | Assist users with web tasks |
| Developers | Test automation | Write browser-based integration tests |
| QA Engineers | End-to-end testing | Automate web application validation |

### Use Cases

**UC-1: Autonomous Web Research**
- An AI agent receives a research task
- It calls `browser_navigate` to load a URL
- It calls `browser_get_text` to extract article content
- It calls `browser_screenshot` to capture visuals
- The agent synthesizes findings without human intervention

**UC-2: Form Auto-Completion**
- An AI agent receives a task to fill a web form
- It calls `browser_navigate` to load the form URL
- It calls `browser_fill_form` with a dictionary of field selectors and values
- The tool handles typing, focus events, and submission
- A screenshot is returned for verification

**UC-3: Dynamic Content Scraping**
- An AI agent needs data from a JavaScript-heavy page
- It calls `browser_navigate` then `browser_wait_for` for dynamic content
- It calls `browser_evaluate` to run custom extraction JavaScript
- Structured data is returned as JSON

**UC-4: Visual Testing**
- A developer writes a test that navigates to a page
- The tool captures a `browser_screenshot` (full-page or element)
- Screenshots are compared against baselines
- Diff images are returned for review

**UC-5: Multi-Page Session Management**
- An AI agent needs to maintain login state across pages
- It creates a `BrowserContext` with persistent session
- All actions within the context share cookies and localStorage
- Context can be cloned for parallel workflows

---

## 2. Problem Statement

### Pain Points

1. **Agent Web Isolation**: Current AI agents cannot interact with the web, limiting them to text-based data
2. **Inconsistent Browser APIs**: Each automation tool (Playwright, Puppeteer, Selenium) has different interfaces
3. **MCP Tool Fragmentation**: No standard protocol for exposing browser automation to AI agents
4. **Headless Complexity**: Running browser automation in headless server environments is error-prone
5. **Screenshot Debugging**: Hard to diagnose agent actions without visual evidence on failure

### Jobs to Be Done

| Job | Priority | Current State | Desired State |
|-----|----------|---------------|---------------|
| Expose browser to agents | P0 | No capability | MCP server with tool interface |
| Standardized tool schema | P0 | Ad-hoc | JSON Schema tool definitions |
| Persistent sessions | P1 | New context per action | Shared BrowserContext |
| Automatic failure capture | P1 | Manual screenshot | Screenshot on every error |
| Multi-browser support | P2 | Playwright only | Playwright + Puppeteer |

---

## 3. User Stories

### Story 1: Agent Web Navigation
> As an AI agent, I want to call `browser_navigate` so that I can load any URL and interact with its content.

**Acceptance Criteria**:
- [ ] `browser_navigate` accepts a URL parameter
- [ ] Page loads within 3 seconds
- [ ] Returns page title and final URL (handles redirects)
- [ ] Handles navigation errors gracefully with error message

### Story 2: Element Interaction
> As an AI agent, I want to click, type, and extract text so that I can interact with web applications.

**Acceptance Criteria**:
- [ ] `browser_click` accepts a CSS selector
- [ ] `browser_type` accepts selector and text, simulates real typing
- [ ] `browser_get_text` returns text content of an element
- [ ] All actions return success/error with optional screenshot

### Story 3: Failure Transparency
> As a developer debugging agent behavior, I want automatic screenshots on errors so that I can see what went wrong.

**Acceptance Criteria**:
- [ ] Failed actions include screenshot in response
- [ ] Error includes selector, page URL, and stack trace
- [ ] Logs are attachable to MCP resource responses
- [ ] Screenshot is base64-encoded in JSON response

### Story 4: Session Persistence
> As an AI agent, I want to maintain login state across multiple page interactions so that I can complete multi-step workflows.

**Acceptance Criteria**:
- [ ] `BrowserContext` persists cookies and localStorage
- [ ] Context ID is passed across tool calls
- [ ] Context can be cloned for parallel branches
- [ ] Max 10 concurrent contexts per session

---

## 4. Requirements

### Functional Requirements

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-1 | MCP server implementing stdio transport | P0 |
| FR-2 | `browser_navigate` tool | P0 |
| FR-3 | `browser_click` tool | P0 |
| FR-4 | `browser_type` tool | P0 |
| FR-5 | `browser_screenshot` tool | P0 |
| FR-6 | `browser_evaluate` tool (JavaScript execution) | P1 |
| FR-7 | `browser_get_text` tool | P0 |
| FR-8 | `browser_fill_form` tool | P1 |
| FR-9 | `browser_wait_for` tool | P1 |
| FR-10 | Playwright as primary browser engine | P0 |
| FR-11 | Automatic screenshot on action failure | P0 |
| FR-12 | MCP resources for page state inspection | P2 |
| FR-13 | Session/context management | P1 |
| FR-14 | Puppeteer as secondary engine | P2 |

### Non-Functional Requirements

| ID | Requirement | Target |
|----|-------------|--------|
| NFR-1 | Browser launch time | < 2 seconds cold start |
| NFR-2 | Page navigation time | < 3 seconds full load |
| NFR-3 | Action latency (click/type) | < 100ms response |
| NFR-4 | Screenshot capture time | < 500ms full page |
| NFR-5 | MCP protocol overhead | < 50ms per request |
| NFR-6 | Memory per page | < 50MB isolated |
| NFR-7 | Concurrent pages | 10 maximum |
| NFR-8 | Session duration | 30 minutes max |

---

## 5. Out of Scope

- Desktop application automation (Electron apps)
- Mobile device emulation (future phase)
- Browser extension injection (future phase)
- WebSocket-based remote browser sessions
- HTTP proxy routing through browser
- Multi-tab coordination beyond context cloning

---

## 6. Dependencies

### External

| Dependency | Purpose | Version |
|------------|---------|---------|
| `mcp` | Model Context Protocol SDK | >= 1.0 |
| `playwright` | Browser automation engine | >= 1.40 |
| `pydantic` | Data validation | >= 2.0 |
| `click` | CLI framework | >= 8.1 |
| `structlog` | Structured logging | >= 24.0 |
| `pytest` / `pytest-asyncio` | Testing | >= 8.0 |

### Internal

- `src/` — Shared MCP utilities
- `tests/` — Test fixtures

---

## 7. Security Considerations

| Concern | Mitigation |
|---------|------------|
| Arbitrary JavaScript execution | `browser_evaluate` restricted to documented page context |
| Credential exposure | No credential storage; sessions are ephemeral |
| Origin restrictions | `allowed_origins` config limits navigable domains |
| Resource exhaustion | `max_pages` limit (10) prevents memory exhaustion |
| Timeout enforcement | 30s default timeout per action |

---

## 8. Success Metrics

| Metric | Baseline | Target | Measurement |
|--------|----------|--------|-------------|
| MCP tool adoption | 0 | 5+ agents using it | Log analysis |
| Action success rate | N/A | > 95% | Success flag in responses |
| Browser crash rate | N/A | < 1% | Error logs |
| Session abandonment | N/A | < 5% | Timeout ratio |
| Screenshot attach rate | N/A | 100% on failure | Error responses |

---

## 9. Release Plan

### Phase 1: Core Browser Tools (MVP)
- MCP server with stdio transport
- Playwright chromium engine
- `browser_navigate`, `browser_click`, `browser_type`, `browser_screenshot`, `browser_get_text`
- Automatic screenshot on failure

### Phase 2: Advanced Interactions
- `browser_evaluate`, `browser_fill_form`, `browser_wait_for`
- Session/context management
- Error log attachment to responses

### Phase 3: Multi-Engine Support
- Puppeteer as secondary engine
- MCP resources for page state
- Configurable browser options (viewport, user agent, proxy)

---

## 10. Open Questions

| Question | Owner | Status |
|----------|-------|--------|
| How to handle CAPTCHA/bot detection? | TBD | Open |
| Should we support authentication OAuth flows? | TBD | Open |
| How to handle file downloads triggered by browser? | TBD | Open |
| Should we support browser recording/playback? | TBD | Future |
| How to limit resource consumption per context? | TBD | Open |
