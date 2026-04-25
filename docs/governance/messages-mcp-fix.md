# Messages MCP — Hang Fix (2026-04-25)

## Symptom

Codex hangs indefinitely at `Starting MCP servers (2/3): messages` whenever
`mac_messages_mcp` is configured (PyPI or `git+https://github.com/carterlasalle/mac_messages_mcp.git`).
Symptoms include 50+ orphan `mac-messages-mcp` Python processes that never
complete the MCP `initialize` handshake. Killing them only buys minutes — Codex
respawns fresh ones that hang the same way.

## Root cause

`mac_messages_mcp` is a thin FastMCP wrapper around the macOS Messages SQLite DB
+ AppleScript. Two upstream fragilities combine:

1. **FastMCP / MCP SDK API drift** — upstream issues
   [#29](https://github.com/carterlasalle/mac_messages_mcp/issues/29) and
   [#20](https://github.com/carterlasalle/mac_messages_mcp/issues/20) document
   the `FastMCP(... description=...)` constructor breakage when paired with
   recent `mcp` SDK releases. The repo has since switched to
   `instructions=` (server.py:34) but FastMCP startup itself still stalls under
   `uvx`-resolved environments — likely a tool-registration call that touches
   the Messages DB or AppleScript at import time and never returns when run
   from a sandboxed `uvx` cache.
2. **`uvx --from git+...` cold-start cost** — every Codex launch re-resolves
   the git ref and PEP 517 builds, multiplying the hang surface area when a
   handshake reply is expected within seconds.

Net effect: the MCP server never writes the `initialize` response to stdout,
Codex's MCP client blocks the whole startup phase.

## Solution chosen

**Replace `mac_messages_mcp` with the existing in-house
`agent-imessage-mcp`** (already installed at
`/Users/kooshapari/.local/bin/agent-imessage-mcp`, distributed via the
`agent-imessage` skill). It speaks MCP-over-stdio natively (no FastMCP, no
`uvx` cold start), exposes a richer toolset (`notify_user`,
`wait_for_user_reply`, `user_status`, presence/action signals, session
tracking), and is locally maintained.

Why this over alternatives:

| Option | Verdict |
|---|---|
| Patch upstream `mac_messages_mcp` and use a fork | Even with the `description=`/`instructions=` fix applied, FastMCP+uvx still stalls. Fix is non-trivial. |
| `@dlants/mcp-imessage` (Node) | Maintained but adds a Node runtime dep and only covers send/read. Less surface than agent-imessage. |
| Build a minimal stdio JSON-RPC stub | Already exists — agent-imessage **is** that stub, plus presence + session features. |

## Standalone verification

```bash
# initialize + tools/list
( printf '%s\n' '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}'
  printf '%s\n' '{"jsonrpc":"2.0","method":"notifications/initialized"}'
  printf '%s\n' '{"jsonrpc":"2.0","id":2,"method":"tools/list"}'
) | /Users/kooshapari/.local/bin/agent-imessage-mcp serve
```

Returns the `initialize` result and 15-tool catalog in <1s. No background procs
left hanging (`ps aux | grep mac-messages-mcp | grep -v grep | wc -l` -> `0`).

## `~/.codex/mcp.json` final state

```json
{
  "mcpServers": {
    "thegent":     { "url": "http://127.0.0.1:3847/mcp", "transport": "http" },
    "codex_apps":  { "url": "http://127.0.0.1:3847/mcp", "transport": "http" },
    "workos-docs": { "command": "npx", "args": ["-y", "@workos/mcp-docs-server"], "transport": "stdio" },
    "messages": {
      "command":   "/Users/kooshapari/.local/bin/agent-imessage-mcp",
      "args":      ["serve"],
      "transport": "stdio",
      "description": "macOS Messages MCP via agent-imessage (replaces broken mac_messages_mcp; provides notify/inbox/wait/status)."
    }
  }
}
```

Backups: `~/.codex/mcp.json.bak` (pre-disable) and `~/.codex/mcp.json.bak2`
(post-disable, pre-replacement).

## Tool surface mapping

| Old `mac_messages_mcp` tool | Replacement |
|---|---|
| `tool_send_message`            | `notify_user`         |
| `get_recent_messages`          | `wait_for_user_reply` (`include_existing=true`) + CLI `agent-imessage inbox` |
| `find_contact`                 | scoped `recipient` enum (`koosha`, `sponsor`) |
| `check_db_access`              | `agent-imessage-mcp doctor` |
| `tool_check_imessage_availability` | `agent-imessage-mcp status` |

## Restore / rollback

If the broken upstream variant is ever needed again:

```bash
cp ~/.codex/mcp.json.bak ~/.codex/mcp.json   # restore disabled state
# or
cp ~/.codex/mcp.json.bak2 ~/.codex/mcp.json  # restore broken-but-configured state
```

## Follow-up (optional, not blocking)

- File a comment on
  [carterlasalle/mac_messages_mcp#29](https://github.com/carterlasalle/mac_messages_mcp/issues/29)
  noting that even after the `description=` -> `instructions=` fix, FastMCP
  still stalls under `uvx --from git+...`. Recommend dropping FastMCP for a
  hand-rolled stdio handler (the pattern used by `agent-imessage-mcp`).
- If a generic, non-Phenotype-coupled Messages MCP is ever needed for sharing,
  extract the stdio core of `agent-imessage-mcp` into a standalone
  `messages-mcp-stub` crate/package.
