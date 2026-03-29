import dpss

def test_reconcile_exact_matches():
    keys = [
        dpss.Transaction("k1", 100),
        dpss.Transaction("k2", 200),
        dpss.Transaction("k3", 300),
    ]
    targets = [
        dpss.Transaction("t1", 200),
        dpss.Transaction("t2", 100),
        dpss.Transaction("t3", 400),
    ]

    result = dpss.reconcile(keys, targets, 5, 5, 0, 10)
    
    assert len(result.matched) == 2
    assert len(result.unmatched_keys) == 1
    assert result.unmatched_keys[0].id == "k3"
    assert len(result.unmatched_targets) == 1
    assert result.unmatched_targets[0].id == "t3"

def test_reconcile_many_to_many():
    keys = [
        dpss.Transaction("k1", 100),
        dpss.Transaction("k2", 200),
        dpss.Transaction("k3", 500),
    ]
    targets = [
        dpss.Transaction("t1", 300),
        dpss.Transaction("t2", 200),
        dpss.Transaction("t3", 300),
    ]

    result = dpss.reconcile(keys, targets, 5, 5) # defaults to tolerance=0, n_candidates=10
    
    assert len(result.unmatched_keys) == 0
    assert len(result.unmatched_targets) == 0
    assert result.summary.matched_amount == 800

def test_reconcile_tolerance():
    keys = [
        dpss.Transaction("k1", 103),
        dpss.Transaction("k2", 198),
    ]
    targets = [
        dpss.Transaction("t1", 100),
        dpss.Transaction("t2", 200),
    ]

    result = dpss.reconcile(keys, targets, 5, 5, 10)
    
    assert len(result.unmatched_keys) == 0
    assert len(result.unmatched_targets) == 0
    assert len(result.matched) == 2
    
    diffs = sorted([m.difference for m in result.matched])
    assert diffs == [-2, 3]

def test_transaction_repr():
    tx = dpss.Transaction("test_id", 1500)
    assert repr(tx) == "Transaction(id='test_id', amount=1500)"
