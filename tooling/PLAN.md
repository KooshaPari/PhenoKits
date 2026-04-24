# PLAN.md — tooling

> Experimental tooling directory for Phenotype ecosystem

---

## Project Summary

**Type**: Experimental Tool Collection  
**Purpose**: Development tools, MCP servers, and utilities that enhance the development experience.

---

## Phase 1: Browser Agent MCP Foundation (Week 1-2)

**Duration**: 2 weeks  
**Focus**: MCP server core and browser automation

### Deliverables
- [ ] MCP server core implementation
- [ ] Tool registry system
- [ ] Resource manager
- [ ] Browser lifecycle management (Playwright)
- [ ] Page context management
- [ ] Action handlers (click, type, navigate)
- [ ] CLI interface
- [ ] Configuration manager
- [ ] Basic test suite

### Resource Estimate
| Role | Hours | Count |
|------|-------|-------|
| Python Engineer | 40h | 1 |
| QA Engineer | 10h | 1 |

---

## Phase 2: Browser Automation Features (Week 3-4)

**Duration**: 2 weeks  
**Focus**: Full browser automation capabilities

### Deliverables
- [ ] Screenshot capture (full page, element)
- [ ] JavaScript evaluation
- [ ] Text extraction
- [ ] Form filling
- [ ] Wait conditions
- [ ] Multi-browser support (Chromium, Firefox, WebKit)
- [ ] Proxy configuration
- [ ] User agent rotation
- [ ] Session persistence

### Resource Estimate
| Role | Hours | Count |
|------|-------|-------|
| Python Engineer | 40h | 1 |
| Frontend Engineer | 10h | 1 |

---

## Phase 3: MCP Protocol Completion (Week 5-6)

**Duration**: 2 weeks  
**Focus**: Full MCP specification compliance

### Deliverables
- [ ] MCP stdio transport
- [ ] MCP HTTP transport
- [ ] MCP WebSocket transport
- [ ] Tool definitions (JSON Schema)
- [ ] Resource endpoints
- [ ] Prompt templates
- [ ] Error handling per MCP spec
- [ ] Logging integration (structlog)
- [ ] Claude Desktop compatibility

### Resource Estimate
| Role | Hours | Count |
|------|-------|-------|
| Python Engineer | 40h | 1 |
| Integration Engineer | 15h | 1 |

---

## Phase 4: Performance & Reliability (Week 7-8)

**Duration**: 2 weeks  
**Focus**: Production-ready reliability

### Deliverables
- [ ] Browser pool management
- [ ] Connection retry logic
- [ ] Circuit breaker pattern
- [ ] Memory leak prevention
- [ ] Session timeout handling
- [ ] Concurrent page limits
- [ ] Performance benchmarks
- [ ] Load testing
- [ ] Stress testing

### Resource Estimate
| Role | Hours | Count |
|------|-------|-------|
| Python Engineer | 40h | 1 |
| DevOps Engineer | 15h | 1 |

---

## Phase 5: Legacy Enforcement Tool (Week 9-10)

**Duration**: 2 weeks  
**Focus**: Second tool in the tooling directory

### Deliverables
- [ ] Legacy code detection
- [ ] Migration path suggestions
- [ ] CI/CD integration
- [ ] Pre-commit hook
- [ ] Configuration schema
- [ ] Rule engine
- [ ] Report generation
- [ ] Documentation

### Resource Estimate
| Role | Hours | Count |
|------|-------|-------|
| Python Engineer | 40h | 1 |
| QA Engineer | 10h | 1 |

---

## Summary

| Metric | Value |
|--------|-------|
| **Total Duration** | 10 weeks |
| **Total Engineer Hours** | 310h |
| **Tools Delivered** | 2 (browser-agent-mcp, legacy-enforcement) |
| **Milestones** | 5 |

---

## Risk Factors

1. **Playwright binary size** → Mitigation: Optional installation
2. **MCP protocol changes** → Mitigation: Version pinning
3. **Browser compatibility** → Mitigation: Feature detection

---

## Dependencies

### External
- mcp (Model Context Protocol SDK)
- playwright
- pydantic
- click
- structlog

### Internal
- src/ (shared MCP utilities)

---

*Last updated: 2026-04-04*
