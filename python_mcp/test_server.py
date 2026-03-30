import pytest
from server import (
    _assess_reconciliation_risk,
    _coerce_transactions,
    suggest_reconciliation_config,
    ReconciliationConfigInput
)

def test_assess_reconciliation_risk():
    cfg = ReconciliationConfigInput(max_key_group_size=4, max_target_group_size=4)
    keys = [{"amount": 1}] * 501
    targets = [{"amount": 1}] * 10
    
    # Should block because largest_side > 500 and max_group > 3
    risk = _assess_reconciliation_risk(keys, targets, cfg)
    assert risk["blocked"] is True
    assert risk["reason"] is not None
    assert "Input is too large" in risk["reason"]
    
    # Should block because smallest_side > 200 and max_group > 3
    keys2 = [{"amount": 1}] * 201
    targets2 = [{"amount": 1}] * 201
    risk2 = _assess_reconciliation_risk(keys2, targets2, cfg)
    assert risk2["blocked"] is True
    assert risk2["reason"] is not None
    assert "Both sides are large" in risk2["reason"]
    
    # Allowed
    cfg_safe = ReconciliationConfigInput(max_key_group_size=3, max_target_group_size=3)
    risk3 = _assess_reconciliation_risk(keys, targets, cfg_safe)
    assert risk3["blocked"] is False
    assert len(risk3["warnings"]) > 0  # Large slice detected

def test_coerce_transactions():
    raw = [{"id": 1, "amount": "100"}]
    coerced = _coerce_transactions(raw)
    assert coerced == [{"id": "1", "amount": 100, "date": None, "description": None}]

def test_suggest_reconciliation_config():
    # Small inputs
    res1 = suggest_reconciliation_config(10, 10, tolerance=0, same_currency=True)
    assert res1["config"]["max_key_group_size"] == 5
    assert len(res1["warnings"]) == 0
    
    # Large inputs
    res2 = suggest_reconciliation_config(600, 10, tolerance=10, same_currency=False)
    assert res2["config"]["max_key_group_size"] == 3
    assert len(res2["warnings"]) == 3 # mixed currency, tolerance, > 500
