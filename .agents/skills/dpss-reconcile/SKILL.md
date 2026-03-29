---
name: dpss-reconcile
description: Use when reconciling bank transactions, ledger lines, or solving many-to-many numeric matching with the dpss MCP tools. Prefer this for subset-sum style matching that must remain deterministic and auditable.
---

Use `dpss` as the numeric source of truth. The model may interpret, normalize, and explain data, but it must not invent final matches without running the solver.

## When to use this skill

- Bank-to-ledger reconciliation
- Payment-to-invoice matching
- Many-to-many amount matching
- Subset-sum debugging for residual balances

## Required operating rules

1. Normalize records into `{ id, amount, date, description }` before reconciliation.
2. Keep amounts in integer minor units when possible.
3. Run strict matching first.
4. Use tolerance only after strict matching leaves residuals.
5. Label any explanation derived from descriptions or dates as a hypothesis unless the numeric solver confirmed it.
6. Never alter signs, currencies, duplicates, or amounts silently.

## Tool selection

- Use `healthcheck` to confirm the MCP server is ready.
- Use `normalize_transactions` when the source rows are CSV-like or column names are messy.
- Use `suggest_reconciliation_config` before large reconciliations or when group sizes are unclear.
- Use `reconcile_transactions` for transaction objects with ids and metadata.
- Use `sequence_matcher` for raw amount arrays when the problem is many-to-many but not yet transaction-shaped.
- Use `find_subset` for a single target against one array.

## Default reconciliation sequence

1. Check whether the source is already normalized.
2. If not, normalize it first and surface any assumptions.
3. Request or infer conservative group limits.
4. Run strict reconciliation with `tolerance = 0`.
5. If residuals remain and fee, FX, or rounding explanations are plausible, rerun with a small tolerance.
6. Summarize:
   - confirmed matched groups
   - unmatched residuals
   - any hypotheses that still need user confirmation

## Output discipline

Always separate:

- confirmed numeric matches
- unmatched transactions
- hypotheses or domain explanations

If there is no confirmed match, say so plainly.
