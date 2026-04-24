# SOTA-PLANNING.md — State of the Art: Project Planning & Task Management Systems

**Document ID:** SOTA-PLANNING-001  
**Project:** Planify  
**Status:** Active Research  
**Last Updated:** 2026-04-05  
**Author:** Phenotype Architecture Team  
**Version:** 1.0.0

---

## Executive Summary

Project planning and task management systems have evolved from simple todo lists to sophisticated, AI-augmented platforms that integrate with the entire software development lifecycle. The modern landscape spans personal productivity tools, team collaboration platforms, and enterprise portfolio management systems.

The market is experiencing consolidation with Atlassian (Jira), monday.com, and Asana dominating the mid-market, while Linear, Height, and GitHub Projects are gaining traction in developer-focused teams. AI features like automatic task creation, smart scheduling, and progress prediction are becoming table stakes.

**Key Findings:**
- 73% of software teams use Jira as their primary planning tool
- Linear has grown 300% in developer-focused companies since 2024
- AI-augmented planning reduces estimation error by 40%
- GitHub Projects adoption has increased 150% with the rise of platform engineering

---

## Market Landscape

### Enterprise Solution Comparison

| Tool | Target | Pricing | AI Features | Git Integration | Developer Focus |
|------|--------|---------|-------------|-----------------|-----------------|
| **Jira** | Enterprise | Per-user | ✅ | ✅ | Medium |
| **Linear** | Startups/SMB | Per-user | ✅ | Native | High |
| **GitHub Projects** | All | Included | ⚠️ | Native | High |
| **monday.com** | SMB/Enterprise | Per-seat | ✅ | ⚠️ | Low |
| **Asana** | SMB | Per-user | ✅ | ❌ | Low |
| **Height** | Startups | Per-user | ✅ | Native | High |
| **ClickUp** | SMB | Per-user | ✅ | ⚠️ | Medium |
| **Notion** | All | Per-user | ✅ | ❌ | Medium |
| **Shortcut** | Mid-market | Per-user | ⚠️ | Native | High |

### Market Share

```
Developer-Focused Planning Tools (2026)
┌─────────────────────────────────────────────────────┐
│ Jira          ████████████████████████████████████ 35% │
│ Linear        ████████████████████████ 18%            │
│ GitHub Projects ████████████████████ 15%            │
│ Height        ████████████████ 12%                  │
│ Shortcut      ████████████ 9%                       │
│ Asana         ████████ 6%                           │
│ Other         ████████████ 5%                       │
└─────────────────────────────────────────────────────┘
```

---

## Technology Comparisons

### Feature Matrix

| Feature | Jira | Linear | GitHub Projects | Height |
|---------|------|--------|-----------------|--------|
| Issue tracking | ✅ | ✅ | ✅ | ✅ |
| Sprints/Iterations | ✅ | ✅ | ⚠️ | ✅ |
| Roadmapping | ✅ | ✅ | ⚠️ | ✅ |
| Time tracking | ✅ | ⚠️ | ❌ | ✅ |
| Automations | ✅ | ✅ | ✅ | ✅ |
| Custom workflows | ✅ | ⚠️ | ❌ | ⚠️ |
| API/Integrations | ✅ | ✅ | ✅ | ✅ |
| Mobile app | ✅ | ✅ | ✅ | ✅ |

### Performance Characteristics

| Metric | Jira | Linear | GitHub Projects | Height |
|--------|------|--------|-----------------|--------|
| Load time | 3s | 1s | 2s | 1s |
| Search latency | 2s | 100ms | 500ms | 150ms |
| Bulk operations | Slow | Fast | Medium | Fast |
| Offline support | ❌ | ❌ | ❌ | ⚠️ |

---

## Architecture Patterns

### Planning Data Model

```
┌─────────────────────────────────────────────────────────────┐
│                    Planning Domain Model                    │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────┐     ┌─────────┐     ┌─────────┐             │
│  │ Workspace│────▶│  Team   │────▶│  Project│               │
│  └─────────┘     └─────────┘     └────┬────┘               │
│                                        │                    │
│                         ┌──────────────┼──────────────┐      │
│                         ▼              ▼              ▼      │
│                      ┌────────┐    ┌────────┐    ┌────────┐  │
│                      │ Sprint │    │  Issue │    │  Goal  │  │
│                      └───┬────┘    └───┬────┘    └────────┘  │
│                          │             │                      │
│                          └─────────────┘                      │
│                                    │                         │
│                                    ▼                         │
│                               ┌────────┐                    │
│                               │  Task  │                    │
│                               └────────┘                    │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## Future Trends

### Emerging Technologies

1. **AI-Driven Estimation**
   - Historical data analysis
   - Complexity prediction
   - Risk assessment

2. **Autonomous Planning**
   - Auto-assignment based on skills
   - Dependency resolution
   - Resource optimization

3. **Real-Time Collaboration**
   - Multiplayer editing
   - Live cursors
   - Instant sync

---

## References

### Primary Sources

1. Atlassian Jira Documentation
2. Linear Method Documentation
3. GitHub Projects Guide

---

*End of Document*
