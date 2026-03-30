# `dpss` MCP skeleton

This directory contains a minimal Claude Code oriented MCP server that wraps the existing Python binding.

## Intended flow

1. Build the local Python binding:

```powershell
uv run maturin develop --features python
```

2. Install the MCP SDK in the environment Claude Code will use:

```powershell
uv add "mcp[cli]"
```

3. Let Claude Code load the project-scoped server from the repository root [`.mcp.json`](../.mcp.json).

## Exposed tools

- `healthcheck`
- `find_subset`
- `sequence_matcher`
- `normalize_transactions`
- `reconcile_transactions`
- `suggest_reconciliation_config`

## Notes

- This is intentionally a thin adapter. Deterministic solving stays inside `dpss`.
- The adapter is the right place for row normalization, configuration hints, and agent-safe warnings.
- `reconcile_transactions` now applies a UX-first safety guard for broad high-risk runs so MCP clients do not appear frozen.
- If a client intentionally wants to bypass that guard, pass `allow_risky_execution=true` in the reconciliation config.
