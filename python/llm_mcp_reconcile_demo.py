from __future__ import annotations

import argparse
import io
import json
import sys
import time
from contextlib import redirect_stderr, redirect_stdout
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "python_mcp"))

import server as dpss_mcp


DEFAULT_DATASET = "llm_reconcile_demo"
LARGE_DATASET_CUTOFF = 200
PREVIEW_LIMIT = 20


def resolve_data_dir(dataset: str) -> Path:
    candidate = Path(dataset)
    if candidate.is_absolute():
        return candidate
    if candidate.exists():
        return candidate.resolve()
    return (ROOT / "examples" / dataset).resolve()


def load_json(data_dir: Path, name: str) -> list[dict[str, Any]]:
    return json.loads((data_dir / name).read_text(encoding="utf-8"))


def ids(items: list[dict[str, Any]]) -> list[str]:
    return [item["id"] for item in items]


def group_signature(group: dict[str, Any]) -> str:
    return f"{ids(group['keys'])} == {ids(group['targets'])} diff={group['difference']}"


def print_header(title: str) -> None:
    print(f"\n=== {title} ===")


def print_json(label: str, payload: Any) -> None:
    print(f"{label}:")
    print(json.dumps(payload, indent=2, ensure_ascii=False))


def quiet_call(func: Any, *args: Any, **kwargs: Any) -> Any:
    with redirect_stdout(io.StringIO()), redirect_stderr(io.StringIO()):
        return func(*args, **kwargs)


def timed_quiet_call(label: str, func: Any, *args: Any, **kwargs: Any) -> tuple[Any, float]:
    started = time.perf_counter()
    result = quiet_call(func, *args, **kwargs)
    elapsed = time.perf_counter() - started
    print(f"{label}: {elapsed:.3f}s")
    return result, elapsed


def preview_strings(values: list[str], limit: int = PREVIEW_LIMIT) -> list[str]:
    if len(values) <= limit:
        return values
    return values[:limit] + [f"... ({len(values) - limit} more)"]


def preview_groups(groups: list[dict[str, Any]], limit: int = PREVIEW_LIMIT) -> list[str]:
    rendered = [group_signature(group) for group in groups[:limit]]
    if len(groups) > limit:
        rendered.append(f"... ({len(groups) - limit} more matches)")
    return rendered


def summarize_buckets(buckets: dict[str, list[dict[str, Any]]], limit: int = PREVIEW_LIMIT) -> dict[str, dict[str, Any]]:
    return {
        bucket: {
            "count": len(items),
            "sample_ids": preview_strings(ids(items), limit),
        }
        for bucket, items in buckets.items()
    }


def infer_bucket(tx: dict[str, Any]) -> str:
    text = (tx.get("description") or "").lower()
    if "acme" in text:
        return "acme"
    if "retail" in text:
        return "retail"
    if "subscription" in text:
        return "subscription"
    if "marketplace" in text:
        return "marketplace"
    if "enterprise" in text:
        return "enterprise"
    if "late transfer" in text:
        return "pending"
    if "unidentified" in text:
        return "unknown"
    return "other"


def bucket_transactions(items: list[dict[str, Any]]) -> dict[str, list[dict[str, Any]]]:
    buckets: dict[str, list[dict[str, Any]]] = {}
    for item in items:
        bucket = infer_bucket(item)
        buckets.setdefault(bucket, []).append(item)
    return buckets


def reconcile_bucket(
    bucket_name: str,
    ledger_items: list[dict[str, Any]],
    bank_items: list[dict[str, Any]],
) -> tuple[list[dict[str, Any]], list[dict[str, Any]], list[dict[str, Any]], float]:
    if not ledger_items or not bank_items:
        return [], list(ledger_items), list(bank_items), 0.0

    tolerance_by_bucket = {
        "retail": 1000,
        "enterprise": 2500,
    }
    config = {
        "max_key_group_size": 3,
        "max_target_group_size": 3,
        "tolerance": 0,
        "n_candidates": 10,
    }
    started = time.perf_counter()

    if len(ledger_items) == 1 and len(bank_items) == 1:
        ledger_item = ledger_items[0]
        bank_item = bank_items[0]
        difference = ledger_item["amount"] - bank_item["amount"]
        if difference == 0:
            elapsed = time.perf_counter() - started
            return (
                [
                    {
                        "keys": [ledger_item],
                        "targets": [bank_item],
                        "key_sum": ledger_item["amount"],
                        "target_sum": bank_item["amount"],
                        "difference": 0,
                        "match_type": "exact",
                    }
                ],
                [],
                [],
                elapsed,
            )

        tolerance = tolerance_by_bucket.get(bucket_name, 0)
        if tolerance > 0 and abs(difference) <= tolerance:
            elapsed = time.perf_counter() - started
            return (
                [
                    {
                        "keys": [ledger_item],
                        "targets": [bank_item],
                        "key_sum": ledger_item["amount"],
                        "target_sum": bank_item["amount"],
                        "difference": difference,
                        "match_type": "tolerance_match",
                    }
                ],
                [],
                [],
                elapsed,
            )

        elapsed = time.perf_counter() - started
        return [], [ledger_item], [bank_item], elapsed

    strict = quiet_call(dpss_mcp.reconcile_transactions, ledger_items, bank_items, config=config)
    matches = list(strict["matched"])
    unmatched_ledger = list(strict["unmatched_keys"])
    unmatched_bank = list(strict["unmatched_targets"])

    tolerance = tolerance_by_bucket.get(bucket_name, 0)
    if tolerance > 0 and unmatched_ledger and unmatched_bank:
        rerun = quiet_call(
            dpss_mcp.reconcile_transactions,
            unmatched_ledger,
            unmatched_bank,
            config={
                "max_key_group_size": 3,
                "max_target_group_size": 3,
                "tolerance": tolerance,
                "n_candidates": 10,
            },
        )
        matches.extend(rerun["matched"])
        unmatched_ledger = rerun["unmatched_keys"]
        unmatched_bank = rerun["unmatched_targets"]

    elapsed = time.perf_counter() - started
    return matches, unmatched_ledger, unmatched_bank, elapsed


def run_demo(data_dir: Path, force_global_strict: bool = False) -> None:
    ledger_rows = load_json(data_dir, "ledger_rows.json")
    bank_rows = load_json(data_dir, "bank_rows.json")
    timing: dict[str, float] = {}

    print_header("LLM Plan Using The Skill")
    print("1. Normalize messy source rows into typed transactions.")
    print("2. Ask the MCP helper for a conservative configuration.")
    print("3. Run strict reconciliation first.")
    print("4. If residuals remain, rerun only the residual slice with tolerance.")
    print("5. Summarize confirmed matches and keep hypotheses separate.")
    print(f"dataset: {data_dir.name}")
    print(f"ledger rows: {len(ledger_rows)}")
    print(f"bank rows: {len(bank_rows)}")

    print_header("Step 1: Normalize Inputs")
    normalized_ledger, timing["normalize_ledger"] = timed_quiet_call(
        "normalize_ledger",
        dpss_mcp.normalize_transactions,
        ledger_rows,
        id_field="ledger_id",
        amount_field="gross_amount",
        date_field="posted_on",
        description_field="memo",
        amounts_are_minor_units=True,
    )
    normalized_bank, timing["normalize_bank"] = timed_quiet_call(
        "normalize_bank",
        dpss_mcp.normalize_transactions,
        bank_rows,
        id_field="bank_ref",
        amount_field="credit",
        date_field="value_date",
        description_field="narrative",
        amounts_are_minor_units=True,
    )
    print_json("normalized_ledger_sample", normalized_ledger["transactions"][:2])
    print_json("normalized_bank_sample", normalized_bank["transactions"][:2])

    print_header("Step 2: Suggest Config")
    suggestion, timing["suggest_config"] = timed_quiet_call(
        "suggest_config",
        dpss_mcp.suggest_reconciliation_config,
        n_keys=len(normalized_ledger["transactions"]),
        n_targets=len(normalized_bank["transactions"]),
        tolerance=1000,
        same_currency=True,
    )
    print_json("config_suggestion", suggestion)

    strict_result: dict[str, Any] | None = None
    largest_side = max(len(normalized_ledger["transactions"]), len(normalized_bank["transactions"]))
    should_run_global_strict = force_global_strict or largest_side <= LARGE_DATASET_CUTOFF

    print_header("Step 3: Global Strict Reconciliation")
    if should_run_global_strict:
        strict_result, timing["global_strict"] = timed_quiet_call(
            "global_strict",
            dpss_mcp.reconcile_transactions,
            normalized_ledger["transactions"],
            normalized_bank["transactions"],
            config={
                "max_key_group_size": 5,
                "max_target_group_size": 5,
                "tolerance": 0,
                "n_candidates": 20,
            },
        )
        print(f"strict matched groups: {len(strict_result['matched'])}")
        for signature in preview_groups(strict_result["matched"]):
            print(f"  confirmed: {signature}")
        print(f"strict unmatched ledger ids: {preview_strings(ids(strict_result['unmatched_keys']))}")
        print(f"strict unmatched bank ids: {preview_strings(ids(strict_result['unmatched_targets']))}")
    else:
        print(
            "Skipped global strict reconciliation because this dataset is large enough to feel frozen in a demo."
        )
        print("Use --force-global-strict if you want to run the full challenge path anyway.")

    print_header("Step 4: LLM Semantic Partitioning")
    ledger_buckets = bucket_transactions(normalized_ledger["transactions"])
    bank_buckets = bucket_transactions(normalized_bank["transactions"])
    print_json("ledger_buckets", summarize_buckets(ledger_buckets))
    print_json("bank_buckets", summarize_buckets(bank_buckets))

    print_header("Step 5: Bucketed MCP Reconciliation")
    final_matches: list[dict[str, Any]] = []
    final_unmatched_ledger: list[dict[str, Any]] = []
    final_unmatched_bank: list[dict[str, Any]] = []

    for bucket in sorted(set(ledger_buckets) | set(bank_buckets)):
        bucket_matches, bucket_unmatched_ledger, bucket_unmatched_bank, bucket_elapsed = reconcile_bucket(
            bucket,
            ledger_buckets.get(bucket, []),
            bank_buckets.get(bucket, []),
        )
        if bucket_matches or bucket_unmatched_ledger or bucket_unmatched_bank:
            print(f"bucket={bucket} ({bucket_elapsed:.3f}s)")
            for signature in preview_groups(bucket_matches):
                print(f"  matched: {signature}")
            if bucket_unmatched_ledger:
                print(f"  unmatched ledger: {preview_strings(ids(bucket_unmatched_ledger))}")
            if bucket_unmatched_bank:
                print(f"  unmatched bank: {preview_strings(ids(bucket_unmatched_bank))}")
        final_matches.extend(bucket_matches)
        final_unmatched_ledger.extend(bucket_unmatched_ledger)
        final_unmatched_bank.extend(bucket_unmatched_bank)

    print_header("Step 6: LLM Interpretation")
    print("Confirmed matches after LLM-guided partitioning:")
    for signature in preview_groups(final_matches):
        print(f"- {signature}")

    print("Open items requiring business explanation:")
    if ids(final_unmatched_ledger):
        print(f"- unmatched ledger ids: {preview_strings(ids(final_unmatched_ledger))}")
    if ids(final_unmatched_bank):
        print(f"- unmatched bank ids: {preview_strings(ids(final_unmatched_bank))}")
    for item in final_unmatched_ledger:
        description = (item.get("description") or "").lower()
        if "late transfer" in description:
            print(f"- hypothesis: {item['id']} may be a timing issue because it is described as a late transfer.")
    for item in final_unmatched_bank:
        description = (item.get("description") or "").lower()
        if "unidentified" in description or "unknown" in description:
            print(f"- hypothesis: {item['id']} looks like an unidentified bank deposit that still needs labeling.")

    print_header("Structured Summary")
    final_summary = {
        "dataset": data_dir.name,
        "global_strict_ran": strict_result is not None,
        "global_strict_match_count": len(strict_result["matched"]) if strict_result is not None else None,
        "bucketed_match_count": len(final_matches),
        "final_unmatched_ledger_ids": ids(final_unmatched_ledger),
        "final_unmatched_bank_ids": ids(final_unmatched_bank),
        "timing_seconds": {key: round(value, 3) for key, value in timing.items()},
    }
    print_json("summary", final_summary)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--dataset",
        default=DEFAULT_DATASET,
        help="Example dataset name under examples/ or an explicit path.",
    )
    parser.add_argument(
        "--force-global-strict",
        action="store_true",
        help="Run the expensive global strict reconciliation even on large datasets.",
    )
    args = parser.parse_args()
    run_demo(resolve_data_dir(args.dataset), force_global_strict=args.force_global_strict)
