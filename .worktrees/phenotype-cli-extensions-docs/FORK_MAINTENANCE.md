# Fork Maintenance Strategy

## Overview

This document tracks all Phenotype forks and their maintenance strategies.

## Fork Registry

| Fork Repo | Upstream | Custom Code | Extension Repo |
|-----------|----------|-------------|---------------|
| colab | blackboardsh/colab | AgilePlus specs, webflow-plugin | phenotype-colab-extensions |
| helios-cli | openai/codex | shell-tool-mcp, kitty-specs, dotagents | phenotype-cli-extensions |
| agentapi-plusplus | (none - original) | N/A | N/A |

## Sync Process

### For colab
```bash
# Add upstream if not exists
git remote add upstream https://github.com/blackboardsh/colab.git

# Sync workflow
git fetch upstream
git checkout main
git merge upstream/main
git push origin main

# Custom code is in phenotype-colab-extensions
# Apply customizations from phenotype-colab-extensions after sync
```

### For helios-cli
```bash
# Sync workflow
git fetch upstream
git checkout main
git merge upstream/main

# Apply phenotype customizations
# Custom code in phenotype-cli-extensions
```

## Extension Points

### colab Extensions
- `src/specs/` - AgilePlus PRD and requirements
- `src/webflow-plugin/` - Custom storage manager
- `.taskfile.yml` - Task automation

### helios-cli Extensions  
- `src/shell-tool-mcp/` - Shell tool MCP server
- `src/kitty-specs/` - AgilePlus spec format
- `src/dotagents/` - Dotfiles agent config
- `src/sdk-typescript/` - SDK additions

## Rollback Plan

If upstream sync breaks things:
1. Revert merge commit
2. Keep phenotype extensions in separate repo
3. Re-apply extensions to working version
