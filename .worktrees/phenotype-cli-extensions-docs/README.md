# phenotype-cli-extensions

**Extension and spec payload** for Phenotype’s **helios-cli** fork (upstream: OpenAI Codex CLI). This repo holds customizations that are merged or layered on top of the fork: `shell-tool-mcp`, `kitty-specs`, SDK TypeScript additions, workflows, and related work packages.

It is **not** a replacement for the full CLI; it tracks **what to preserve** when syncing upstream.

## Maintenance

- **`FORK_MAINTENANCE.md`** — fork registry (colab, helios-cli, agentapi) and sync notes  
- **`UPSTREAM_SYNC.md`** — merge steps for `KooshaPari/helios-cli` ← `openai/codex`

## Layout (high level)

| Path | Purpose |
|------|---------|
| `src/shell-tool-mcp/` | Shell tool MCP server |
| `src/kitty-specs/` | AgilePlus / modular-architecture specs |
| `src/sdk-typescript/` | SDK extensions |

## Related repos

- **`phenotype-colab-extensions`** — parallel pattern for the colab fork

## License

See upstream and local `LICENSE` if present.
