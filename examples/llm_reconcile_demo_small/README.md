Small smoke-test dataset for `dpss-reconcile`.

Use this when you want to validate the MCP tools and skill flow without waiting on the large challenge dataset in `examples/llm_reconcile_demo`.

Properties:
- 9 ledger rows
- 7 bank rows
- includes exact matches, many-to-one matching, many-to-many matching, one fee/tolerance case, and unmatched residuals

Suggested command:

```powershell
.\.venv\Scripts\python.exe python\llm_mcp_reconcile_demo.py --dataset llm_reconcile_demo_small
```
