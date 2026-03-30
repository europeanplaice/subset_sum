import pytest
import dpss
from server import (
    healthcheck,
    find_subset,
    sequence_matcher,
    reconcile_transactions,
    normalize_transactions,
    _make_dpss_transactions,
    _tx_to_dict,
    _build_unmatched_only_result,
    _result_to_dict
)

def test_healthcheck():
    res = healthcheck()
    assert res["server"] == "dpss"
    assert "find_subset" in res["tools"]
    assert res["python_binding_version"] != "unknown"

def test_find_subset():
    res = find_subset([1, 2, 3], 5, 2)
    assert res["target"] == 5
    assert res["max_length"] == 2
    assert [2, 3] in res["solutions"] or (2, 3) in res["solutions"] or [3, 2] in res["solutions"] or (3, 2) in res["solutions"]

def test_sequence_matcher():
    res = sequence_matcher([10, 20], [30], 2, 1)
    assert "candidates" in res
    assert len(res["candidates"]) > 0
    candidate = res["candidates"][0]
    assert "matched_groups" in candidate

def test_reconcile_transactions():
    keys = [{"id": "k1", "amount": 100}, {"id": "k2", "amount": 50}]
    targets = [{"id": "t1", "amount": 150}]
    
    res = reconcile_transactions(keys, targets)
    assert res["status"] == "completed"
    assert "summary" in res
    assert res["summary"]["matched_key_count"] == 2
    assert res["summary"]["matched_target_count"] == 1
    assert res["summary"]["matched_amount"] == 150
    assert len(res["matched"]) == 1

def test_reconcile_transactions_blocked():
    keys = [{"id": str(i), "amount": 1} for i in range(501)]
    targets = [{"id": "t1", "amount": 1}]
    
    # Blocked due to large size and default max_group_size=5
    res = reconcile_transactions(keys, targets)
    assert res["status"] == "blocked"
    assert res["summary"]["matched_key_count"] == 0
    assert res["summary"]["unmatched_key_amount"] == 501
    assert "guard" in res
    assert "reason" in res["guard"]

def test_normalize_transactions_tool():
    rows = [{"amount": "12.34"}]
    res = normalize_transactions(rows, amounts_are_minor_units=False)
    assert res["count"] == 1
    assert res["transactions"][0]["amount"] == 1234
    assert res["amounts_are_minor_units"] is False

def test_make_dpss_transactions():
    items = [{"id": "1", "amount": 100}]
    txs = _make_dpss_transactions(dpss, items)
    assert len(txs) == 1
    assert txs[0].id == "1"
    assert txs[0].amount == 100

def test_tx_to_dict():
    tx = dpss.Transaction("1", 100, "2023-01-01", "test")
    d = _tx_to_dict(tx)
    assert d == {"id": "1", "amount": 100, "date": "2023-01-01", "description": "test"}

def test_build_unmatched_only_result():
    from schemas import ReconciliationConfigInput
    keys = [{"id": "k1", "amount": 100}]
    targets = [{"id": "t1", "amount": 200}]
    cfg = ReconciliationConfigInput()
    
    res = _build_unmatched_only_result(
        keys, targets, cfg,
        status="blocked",
        warnings=["w1"],
        reason="r1",
        next_steps=["n1"]
    )
    assert res["status"] == "blocked"
    assert res["summary"]["unmatched_key_amount"] == 100
    assert res["summary"]["unmatched_target_amount"] == 200
    assert res["warnings"] == ["w1"]
    assert res["guard"]["reason"] == "r1"
    assert res["guard"]["next_steps"] == ["n1"]
