# Upstream Sync Strategy

## Fork: KooshaPari/helios-cli ← openai/codex

### Sync Process

1. Fetch upstream changes
2. Merge into main
3. Apply phenotype extensions
4. Apply shell-tool-mcp patches
5. Test
6. Push to origin

### Extension Points

Custom code preserved during sync:
- `src/shell-tool-mcp/` - Shell tool MCP server
- `src/kitty-specs/` - AgilePlus spec format
- `src/workflows/` - Phenotype CI workflows
- `src/dotagents/` - Dotfiles agent config
- `src/sdk-typescript/` - SDK additions
