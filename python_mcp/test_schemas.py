from schemas import TransactionInput, ReconciliationConfigInput, _optional_str

def test_transaction_input():
    raw = {"id": 1, "amount": "100", "date": " 2023-01-01 ", "description": ""}
    tx = TransactionInput.from_dict(raw)
    assert tx.id == "1"
    assert tx.amount == 100
    assert tx.date == "2023-01-01"
    assert tx.description is None
    
    assert tx.to_dict() == {
        "id": "1",
        "amount": 100,
        "date": "2023-01-01",
        "description": None
    }

def test_reconciliation_config_input():
    cfg = ReconciliationConfigInput.from_dict(None)
    assert cfg.max_key_group_size == 5
    assert cfg.tolerance == 0
    assert not cfg.allow_risky_execution
    
    cfg2 = ReconciliationConfigInput.from_dict({"max_key_group_size": 10, "tolerance": 5, "allow_risky_execution": True})
    assert cfg2.max_key_group_size == 10
    assert cfg2.tolerance == 5
    assert cfg2.allow_risky_execution is True
    
    assert cfg2.to_dict() == {
        "max_key_group_size": 10,
        "max_target_group_size": 5,
        "tolerance": 5,
        "n_candidates": 10,
        "allow_risky_execution": True
    }

def test_optional_str():
    assert _optional_str(None) is None
    assert _optional_str("  ") is None
    assert _optional_str(" foo ") == "foo"
