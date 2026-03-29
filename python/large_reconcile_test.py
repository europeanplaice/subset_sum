import dpss
import random
import time

def generate_transactions(n, prefix, min_val=10, max_val=1000):
    return [dpss.Transaction(f"{prefix}_{i}", random.randint(min_val, max_val)) for i in range(n)]

def test_large_reconciliation(num_keys, num_targets, max_k_size, max_t_size):
    print(f"\n--- Testing Reconcile ---")
    print(f"Keys: {num_keys}, Targets: {num_targets}, Max Key Group: {max_k_size}, Max Target Group: {max_t_size}")
    
    # We will make some explicit matches
    keys = []
    targets = []
    
    # create 10% explicit matches of 1-to-1
    for i in range(num_keys // 10):
        val = random.randint(100, 500)
        keys.append(dpss.Transaction(f"k_m_{i}", val))
        targets.append(dpss.Transaction(f"t_m_{i}", val))
        
    # create some 2-to-1 matches
    for i in range(num_keys // 20):
        val1 = random.randint(50, 200)
        val2 = random.randint(50, 200)
        keys.append(dpss.Transaction(f"k_m2_{i}_1", val1))
        keys.append(dpss.Transaction(f"k_m2_{i}_2", val2))
        targets.append(dpss.Transaction(f"t_m1_{i}", val1 + val2))
        
    # fill the rest with noise
    while len(keys) < num_keys:
        keys.append(dpss.Transaction(f"k_n_{len(keys)}", random.randint(10, 1000)))
        
    while len(targets) < num_targets:
        targets.append(dpss.Transaction(f"t_n_{len(targets)}", random.randint(10, 1000)))
        
    random.shuffle(keys)
    random.shuffle(targets)
    
    start_time = time.time()
    result = dpss.reconcile(keys, targets, max_k_size, max_t_size, tolerance=0, n_candidates=10)
    end_time = time.time()
    
    print(f"Time taken: {end_time - start_time:.4f} seconds")
    print(f"Matched Groups: {len(result.matched)}")
    print(f"Unmatched Keys: {len(result.unmatched_keys)}")
    print(f"Unmatched Targets: {len(result.unmatched_targets)}")
    print(f"Total Matches Value: {result.summary.matched_amount}")

if __name__ == "__main__":
    random.seed(42)
    # 1. 100 keys, 100 targets, max_group 2
    test_large_reconciliation(100, 100, 2, 2)
    
    # 2. 500 keys, 500 targets, max_group 3
    test_large_reconciliation(500, 500, 3, 3)

    # 3. 1000 keys, 1000 targets, max_group 3
    test_large_reconciliation(1000, 1000, 3, 3)
    
    # 4. 2000 keys, 2000 targets, max_group 4
    test_large_reconciliation(2000, 2000, 4, 4)
