from __future__ import annotations

import importlib
from typing import Any

from mcp.server.fastmcp import FastMCP

from normalize import normalize_transaction_rows
from schemas import ReconciliationConfigInput, TransactionInput


mcp = FastMCP(
    "dpss",
    instructions=(
        "Deterministic subset-sum and transaction reconciliation tools backed by the "
        "dpss solver. Use these tools for numeric matching, not for speculative inference."
    ),
)


def _error_response(message: str) -> dict[str, Any]:
    """Build a structured error payload instead of raising an exception."""
    return {"error": True, "message": message}


def _load_dpss() -> Any:
    try:
        return importlib.import_module("dpss")
    except ModuleNotFoundError as exc:
        raise RuntimeError(
            "The Python binding `dpss` is not installed. Build it first, for example with "
            "`uv run maturin develop --features python`, then restart the MCP server."
        ) from exc


def _coerce_transactions(items: list[dict[str, Any]]) -> list[dict[str, Any]]:
    return [TransactionInput.from_dict(item).to_dict() for item in items]


def _make_dpss_transactions(module: Any, items: list[dict[str, Any]]) -> list[Any]:
    return [
        module.Transaction(
            item["id"],
            item["amount"],
            item.get("date"),
            item.get("description"),
        )
        for item in items
    ]


def _result_to_dict(result: Any) -> dict[str, Any]:
    return {
        "matched": [
            {
                "keys": [_tx_to_dict(tx) for tx in group.keys],
                "targets": [_tx_to_dict(tx) for tx in group.targets],
                "key_sum": group.key_sum,
                "target_sum": group.target_sum,
                "difference": group.difference,
                "match_type": "exact" if group.difference == 0 else "tolerance_match",
            }
            for group in result.matched
        ],
        "unmatched_keys": [_tx_to_dict(tx) for tx in result.unmatched_keys],
        "unmatched_targets": [_tx_to_dict(tx) for tx in result.unmatched_targets],
        "summary": {
            "total_keys": result.summary.total_keys,
            "total_targets": result.summary.total_targets,
            "matched_key_count": result.summary.matched_key_count,
            "matched_target_count": result.summary.matched_target_count,
            "matched_amount": result.summary.matched_amount,
            "unmatched_key_amount": result.summary.unmatched_key_amount,
            "unmatched_target_amount": result.summary.unmatched_target_amount,
        },
    }


def _build_unmatched_only_result(
    keys: list[dict[str, Any]],
    targets: list[dict[str, Any]],
    cfg: ReconciliationConfigInput,
    *,
    status: str,
    warnings: list[str],
    reason: str,
    next_steps: list[str],
) -> dict[str, Any]:
    return {
        "status": status,
        "matched": [],
        "unmatched_keys": list(keys),
        "unmatched_targets": list(targets),
        "summary": {
            "total_keys": len(keys),
            "total_targets": len(targets),
            "matched_key_count": 0,
            "matched_target_count": 0,
            "matched_amount": 0,
            "unmatched_key_amount": sum(item["amount"] for item in keys),
            "unmatched_target_amount": sum(item["amount"] for item in targets),
        },
        "warnings": warnings,
        "config": cfg.to_dict(),
        "guard": {
            "reason": reason,
            "next_steps": next_steps,
        },
    }


def _assess_reconciliation_risk(
    keys: list[dict[str, Any]],
    targets: list[dict[str, Any]],
    cfg: ReconciliationConfigInput,
) -> dict[str, Any]:
    n_keys = len(keys)
    n_targets = len(targets)
    largest_side = max(n_keys, n_targets)
    smallest_side = min(n_keys, n_targets)
    max_group = max(cfg.max_key_group_size, cfg.max_target_group_size)

    warnings: list[str] = []
    next_steps = [
        "Partition by account, memo, currency, or date window before retrying.",
        "Reduce max_key_group_size and max_target_group_size to 3 or less for large slices.",
        "Run strict reconciliation on smaller slices first, then use tolerance only on residuals.",
    ]

    if largest_side > 500 and max_group > 3:
        warnings.append(
            "Blocked a high-risk reconciliation request before execution to avoid a long-running freeze."
        )
        return {
            "blocked": True,
            "reason": (
                f"Input is too large for max group size {max_group} "
                f"({n_keys} keys, {n_targets} targets)."
            ),
            "warnings": warnings,
            "next_steps": next_steps,
        }

    if smallest_side > 200 and max_group > 3:
        warnings.append(
            "Blocked a broad many-to-many search that is likely to feel frozen in an MCP session."
        )
        return {
            "blocked": True,
            "reason": (
                f"Both sides are large ({n_keys} keys, {n_targets} targets) and group size {max_group} "
                "widens the search space too much."
            ),
            "warnings": warnings,
            "next_steps": next_steps,
        }

    if largest_side > 500:
        warnings.append(
            "Large slice detected. This run is allowed, but you should expect slower responses unless the data is partitioned."
        )

    return {
        "blocked": False,
        "reason": None,
        "warnings": warnings,
        "next_steps": next_steps,
    }


def _tx_to_dict(tx: Any) -> dict[str, Any]:
    return {
        "id": tx.id,
        "amount": tx.amount,
        "date": getattr(tx, "date", None),
        "description": getattr(tx, "description", None),
    }


@mcp.tool()
def healthcheck() -> dict[str, Any]:
    """Check whether the dpss MCP server is ready and return version info.

    Call this before any other tool to confirm that the Rust-backed Python
    binding is installed and loadable.

    Returns:
        server: Always "dpss".
        python_binding_version: Installed package version string.
        tools: List of available tool names on this server.
    """
    try:
        module = _load_dpss()
    except RuntimeError as exc:
        return _error_response(str(exc))
    return {
        "server": "dpss",
        "python_binding_version": getattr(module, "__version__", "unknown"),
        "tools": [
            "find_subset",
            "sequence_matcher",
            "reconcile_transactions",
            "normalize_transactions",
            "suggest_reconciliation_config",
        ],
    }


@mcp.tool()
def find_subset(arr: list[int], target: int, max_length: int) -> dict[str, Any]:
    """Find all subsets of `arr` whose elements sum exactly to `target`.

    Use this when you have a single array of integers and need to know which
    combinations add up to a specific value. Negative values are supported.

    Args:
        arr: List of integers to search (may include negatives).
        target: The exact sum each returned subset must equal.
        max_length: Maximum number of elements allowed in each subset.
            Smaller values run faster; start with 3-5.

    Returns:
        solutions: List of subsets (each a list[int]) that sum to target.
        target: Echo of the requested target.
        max_length: Echo of the requested max_length.

    Example:
        find_subset([1, -2, 3, 4, 5], 2, 3) -> solutions: [[4, -2], [3, -2, 1]]
    """
    if not arr:
        return _error_response("arr must be a non-empty list of integers.")
    if max_length <= 0:
        return _error_response("max_length must be a positive integer.")
    try:
        module = _load_dpss()
        return {
            "solutions": module.find_subset(arr, target, max_length),
            "target": target,
            "max_length": max_length,
        }
    except RuntimeError as exc:
        return _error_response(str(exc))
    except (TypeError, ValueError, OverflowError) as exc:
        return _error_response(f"Invalid input: {exc}")


@mcp.tool()
def sequence_matcher(
    keys: list[int],
    targets: list[int],
    max_key_length: int,
    max_target_length: int,
    n_candidates: int = 10,
    use_all_keys: bool = False,
    use_all_targets: bool = False,
) -> dict[str, Any]:
    """Find many-to-many numeric matches between two integer arrays.

    Pairs subsets from `keys` with subsets from `targets` so that each pair
    sums to the same value. Use this for raw numeric matching when the data
    is not yet structured as transactions (no ids/metadata).

    Args:
        keys: First array of integers.
        targets: Second array of integers.
        max_key_length: Max elements from keys in a single matched group.
        max_target_length: Max elements from targets in a single matched group.
        n_candidates: Number of candidate solutions to return (default 10).
        use_all_keys: If true, every element of keys must appear in the solution.
        use_all_targets: If true, every element of targets must appear in the solution.

    Returns:
        candidates: List of candidate solutions. Each candidate contains:
            matched_groups: List of {keys, targets, difference} pairs.
            keys_remainder: Unmatched integers from keys.
            targets_remainder: Unmatched integers from targets.
    """
    if not keys or not targets:
        return _error_response("Both keys and targets must be non-empty lists.")
    if max_key_length <= 0 or max_target_length <= 0:
        return _error_response("max_key_length and max_target_length must be positive integers.")
    try:
        module = _load_dpss()
        raw = module.sequence_matcher(
            keys,
            targets,
            max_key_length,
            max_target_length,
            n_candidates,
            use_all_keys,
            use_all_targets,
        )
        candidates = [
            {
                "matched_groups": [
                    {"keys": pair[0], "targets": pair[1], "difference": sum(pair[0]) - sum(pair[1])}
                    for pair in answer[0]
                ],
                "keys_remainder": answer[1],
                "targets_remainder": answer[2],
            }
            for answer in raw
        ]
        return {"candidates": candidates}
    except RuntimeError as exc:
        return _error_response(str(exc))
    except (TypeError, ValueError, OverflowError) as exc:
        return _error_response(f"Invalid input: {exc}")


@mcp.tool()
def reconcile_transactions(
    keys: list[dict[str, Any]],
    targets: list[dict[str, Any]],
    config: dict[str, Any] | None = None,
) -> dict[str, Any]:
    """Match two sets of transactions, finding many-to-many correspondences.

    This is the primary reconciliation tool. It finds groups of key transactions
    whose summed amounts match groups of target transactions (within an optional
    tolerance), then reports matched groups and unmatched residuals.

    Run normalize_transactions first if your source rows have non-standard
    column names or decimal amounts. Call suggest_reconciliation_config first
    if you are unsure about safe group-size limits.

    Args:
        keys: List of transaction dicts. Required fields: id (str), amount (int,
            minor units). Optional: date (str), description (str).
        targets: Same schema as keys.
        config: Optional dict with:
            max_key_group_size (int, default 5): Max keys in one matched group.
            max_target_group_size (int, default 5): Max targets in one matched group.
            tolerance (int, default 0): Allowed difference between group sums.
                Use 0 for strict matching; use a small value (e.g. 500 = $5.00)
                for fee/rounding tolerance. Always try 0 first.
            n_candidates (int, default 10): Solver candidate count.
            allow_risky_execution (bool, default false): Bypass the safety guard
                that blocks large-input + large-group-size combinations.

    Returns:
        status: "completed" or "blocked" (if safety guard triggered).
        matched: List of matched groups, each with keys, targets, key_sum,
            target_sum, difference, and match_type ("exact" | "tolerance_match").
        unmatched_keys: Transactions from keys that were not matched.
        unmatched_targets: Transactions from targets that were not matched.
        summary: Aggregate counts and amounts.
        warnings: List of advisory messages (e.g. tolerance enabled, residuals remain).
        config: Echo of the reconciliation config used.
    """
    if not keys and not targets:
        return _error_response("Both keys and targets are empty. Provide at least one side.")
    if not keys or not targets:
        cfg = ReconciliationConfigInput.from_dict(config)
        return _build_unmatched_only_result(
            _coerce_transactions(keys) if keys else [],
            _coerce_transactions(targets) if targets else [],
            cfg,
            status="completed",
            warnings=["One side is empty; nothing to match."],
            reason="One side is empty.",
            next_steps=["Provide transactions on both sides to run reconciliation."],
        )
    try:
        module = _load_dpss()
        normalized_keys = _coerce_transactions(keys)
        normalized_targets = _coerce_transactions(targets)
        cfg = ReconciliationConfigInput.from_dict(config)
    except RuntimeError as exc:
        return _error_response(str(exc))
    except (KeyError, TypeError, ValueError) as exc:
        return _error_response(f"Invalid transaction data: {exc}")

    risk = _assess_reconciliation_risk(normalized_keys, normalized_targets, cfg)

    if risk["blocked"] and not cfg.allow_risky_execution:
        return _build_unmatched_only_result(
            normalized_keys,
            normalized_targets,
            cfg,
            status="blocked",
            warnings=risk["warnings"],
            reason=risk["reason"],
            next_steps=risk["next_steps"],
        )

    try:
        result = module.reconcile(
            _make_dpss_transactions(module, normalized_keys),
            _make_dpss_transactions(module, normalized_targets),
            cfg.max_key_group_size,
            cfg.max_target_group_size,
            cfg.tolerance,
            cfg.n_candidates,
        )
    except (ValueError, RuntimeError) as exc:
        return _error_response(f"Solver failed: {exc}")

    payload = _result_to_dict(result)
    warnings = list(risk["warnings"])
    if cfg.tolerance > 0:
        warnings.append("Tolerance-based matching was enabled. Review non-zero differences.")
    if payload["unmatched_keys"] or payload["unmatched_targets"]:
        warnings.append("Residual unmatched transactions remain. Consider business rules or a narrower partition.")
    payload["warnings"] = warnings
    payload["config"] = cfg.to_dict()
    payload["status"] = "completed"
    return payload


@mcp.tool()
def normalize_transactions(
    rows: list[dict[str, Any]],
    id_field: str = "id",
    amount_field: str = "amount",
    date_field: str = "date",
    description_field: str = "description",
    amounts_are_minor_units: bool = True,
) -> dict[str, Any]:
    """Convert CSV-like rows into the {id, amount, date, description} schema
    expected by reconcile_transactions.

    Use this when your source data has non-standard column names (e.g.
    "Transaction ID", "Debit") or when amounts are in decimal major units
    (e.g. 103.50) that need conversion to integer minor units (10350).

    Args:
        rows: List of dicts representing raw rows.
        id_field: Key in each row to use as the transaction id (default "id").
            If missing or empty, an auto-generated "row-N" id is assigned.
        amount_field: Key for the amount value (default "amount").
        date_field: Key for the date value (default "date").
        description_field: Key for the description value (default "description").
        amounts_are_minor_units: If true (default), amounts are already integers
            in minor units and are passed through as-is. If false, amounts are
            treated as decimal major units and multiplied by 100.

    Returns:
        transactions: List of normalized {id, amount, date, description} dicts.
        count: Number of transactions produced.
        amounts_are_minor_units: Echo of the flag used.
    """
    if not rows:
        return _error_response("rows must be a non-empty list of dicts.")
    try:
        normalized = normalize_transaction_rows(
            rows,
            id_field=id_field,
            amount_field=amount_field,
            date_field=date_field,
            description_field=description_field,
            amounts_are_minor_units=amounts_are_minor_units,
        )
    except (KeyError, TypeError, ValueError) as exc:
        return _error_response(f"Normalization failed: {exc}")
    return {
        "transactions": normalized,
        "count": len(normalized),
        "amounts_are_minor_units": amounts_are_minor_units,
    }


@mcp.tool()
def suggest_reconciliation_config(
    n_keys: int,
    n_targets: int,
    tolerance: int = 0,
    same_currency: bool = True,
) -> dict[str, Any]:
    """Suggest safe reconciliation config based on dataset size.

    Call this before reconcile_transactions when you are unsure what
    max_key_group_size / max_target_group_size to use. It returns conservative
    defaults that avoid combinatorial explosion on large inputs.

    Args:
        n_keys: Number of key-side transactions.
        n_targets: Number of target-side transactions.
        tolerance: Intended tolerance in minor units (default 0).
        same_currency: Whether all transactions share the same currency
            (default true). Set to false to receive a currency-partitioning warning.

    Returns:
        config: Recommended {max_key_group_size, max_target_group_size,
            tolerance, n_candidates} dict, ready to pass to reconcile_transactions.
        warnings: Advisory messages (e.g. partition advice for large sets).
    """
    if n_keys <= 0 or n_targets <= 0:
        return _error_response("n_keys and n_targets must be positive integers.")
    max_group = 3 if max(n_keys, n_targets) > 200 else 5
    n_candidates = 20 if max(n_keys, n_targets) <= 50 else 10
    warnings: list[str] = []

    if max(n_keys, n_targets) > 500:
        warnings.append("Large input set. Partition by account, date window, or memo before full reconciliation.")
    if not same_currency:
        warnings.append("Mixed currency detected. Normalize FX or partition by currency before matching.")
    if tolerance > 0:
        warnings.append("Prefer strict matching first, then rerun with tolerance only for residuals.")

    return {
        "config": {
            "max_key_group_size": max_group,
            "max_target_group_size": max_group,
            "tolerance": tolerance,
            "n_candidates": n_candidates,
        },
        "warnings": warnings,
    }


def main() -> None:
    mcp.run()


if __name__ == "__main__":
    main()
