use crate::dp::sequence_matcher;
use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub amount: i64,
    pub date: Option<String>,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReconciliationConfig {
    pub max_key_group_size: usize,
    pub max_target_group_size: usize,
    pub tolerance: i64, // tolerance: 0 means exact match
    pub n_candidates: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MatchedGroup {
    pub keys: Vec<Transaction>,
    pub targets: Vec<Transaction>,
    pub key_sum: i64,
    pub target_sum: i64,
    pub difference: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReconciliationResult {
    pub matched: Vec<MatchedGroup>,
    pub unmatched_keys: Vec<Transaction>,
    pub unmatched_targets: Vec<Transaction>,
    pub summary: ReconciliationSummary,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReconciliationSummary {
    pub total_keys: usize,
    pub total_targets: usize,
    pub matched_key_count: usize,
    pub matched_target_count: usize,
    pub matched_amount: i64,
    pub unmatched_key_amount: i64,
    pub unmatched_target_amount: i64,
}

pub fn reconcile(
    keys: Vec<Transaction>,
    targets: Vec<Transaction>,
    config: ReconciliationConfig,
) -> Result<ReconciliationResult, String> {
    let mut matched_groups = Vec::new();
    let total_keys_count = keys.len();
    let total_targets_count = targets.len();
    
    let round = |amt: i64| -> i64 {
        if config.tolerance <= 0 {
            amt
        } else {
            let half = config.tolerance / 2;
            if amt >= 0 {
                ((amt + half) / config.tolerance) * config.tolerance
            } else {
                ((amt - half) / config.tolerance) * config.tolerance
            }
        }
    };

    // Pass 1: 1-to-1 exact match
    let mut remaining_keys = Vec::new();
    let mut target_map: HashMap<i64, Vec<Transaction>> = HashMap::new();
    
    for target in targets.clone() {
        target_map.entry(round(target.amount)).or_default().push(target);
    }
    
    let mut matched_target_ids = HashSet::new();
    let mut matched_key_ids = HashSet::new();

    for key in keys.clone() {
        let rounded_key = round(key.amount);
        if let Some(target_list) = target_map.get_mut(&rounded_key) {
            if let Some(target) = target_list.pop() {
                matched_groups.push(MatchedGroup {
                    keys: vec![key.clone()],
                    targets: vec![target.clone()],
                    key_sum: key.amount,
                    target_sum: target.amount,
                    difference: key.amount - target.amount,
                });
                matched_target_ids.insert(target.id.clone());
                matched_key_ids.insert(key.id.clone());
                if target_list.is_empty() {
                    target_map.remove(&rounded_key);
                }
                continue;
            }
        }
        remaining_keys.push(key);
    }

    let mut remaining_targets = Vec::new();
    for target in targets.clone() {
        if !matched_target_ids.contains(&target.id) {
            remaining_targets.push(target);
        }
    }

    // Pass 1.5: Greedy N-to-1 and 1-to-N matching
    // Helper to find a single subset that sums to a target value
    fn find_subset(
        pool: &[Transaction],
        target_sum: i64,
        max_depth: usize,
        round_fn: &impl Fn(i64) -> i64,
    ) -> Option<Vec<usize>> {
        fn dfs(
            pool: &[Transaction],
            target_sum: i64,
            max_depth: usize,
            round_fn: &impl Fn(i64) -> i64,
            start_idx: usize,
            depth: usize,
            current_sum: i64,
            path: &mut Vec<usize>,
        ) -> Option<Vec<usize>> {
            if depth > 0 && round_fn(current_sum) == target_sum {
                return Some(path.clone());
            }
            if depth == max_depth {
                return None;
            }
            for i in start_idx..pool.len() {
                path.push(i);
                if let Some(res) = dfs(
                    pool,
                    target_sum,
                    max_depth,
                    round_fn,
                    i + 1,
                    depth + 1,
                    current_sum + pool[i].amount,
                    path,
                ) {
                    return Some(res);
                }
                path.pop();
            }
            None
        }
        let mut path = Vec::with_capacity(max_depth);
        dfs(pool, target_sum, max_depth, round_fn, 0, 0, 0, &mut path)
    }

    // 1-to-N
    if config.max_target_group_size > 1 {
        let mut new_remaining_keys = Vec::new();
        for key in remaining_keys {
            if let Some(indices) = find_subset(
                &remaining_targets,
                round(key.amount),
                config.max_target_group_size,
                &round,
            ) {
                let mut matched_t = Vec::new();
                let mut target_sum = 0;
                // Sort indices descending to remove them safely
                let mut sorted_indices = indices.clone();
                sorted_indices.sort_by(|a, b| b.cmp(a));
                
                for idx in sorted_indices {
                    let t = remaining_targets.remove(idx);
                    target_sum += t.amount;
                    matched_target_ids.insert(t.id.clone());
                    matched_t.push(t);
                }
                matched_key_ids.insert(key.id.clone());
                matched_groups.push(MatchedGroup {
                    keys: vec![key.clone()],
                    targets: matched_t,
                    key_sum: key.amount,
                    target_sum,
                    difference: key.amount - target_sum,
                });
            } else {
                new_remaining_keys.push(key);
            }
        }
        remaining_keys = new_remaining_keys;
    }

    // N-to-1
    if config.max_key_group_size > 1 {
        let mut new_remaining_targets = Vec::new();
        for target in remaining_targets {
            if let Some(indices) = find_subset(
                &remaining_keys,
                round(target.amount),
                config.max_key_group_size,
                &round,
            ) {
                let mut matched_k = Vec::new();
                let mut key_sum = 0;
                // Sort indices descending to remove them safely
                let mut sorted_indices = indices.clone();
                sorted_indices.sort_by(|a, b| b.cmp(a));
                
                for idx in sorted_indices {
                    let k = remaining_keys.remove(idx);
                    key_sum += k.amount;
                    matched_key_ids.insert(k.id.clone());
                    matched_k.push(k);
                }
                matched_target_ids.insert(target.id.clone());
                matched_groups.push(MatchedGroup {
                    keys: matched_k,
                    targets: vec![target.clone()],
                    key_sum,
                    target_sum: target.amount,
                    difference: key_sum - target.amount,
                });
            } else {
                new_remaining_targets.push(target);
            }
        }
        remaining_targets = new_remaining_targets;
    }


    // Pass 2: Combinatorial match (many-to-many)
    let mut key_amounts: Vec<i64> = remaining_keys.iter().map(|k| round(k.amount)).collect();
    let mut target_amounts: Vec<i64> = remaining_targets.iter().map(|t| round(t.amount)).collect();

    // Only run sequence_matcher if there are very few items left, or if max depth is small,
    // to avoid combinatorial explosion.
    if !key_amounts.is_empty() && !target_amounts.is_empty() && key_amounts.len() < 50 && target_amounts.len() < 50 {
        if let Ok(mut answers) = sequence_matcher(
            &mut key_amounts,
            &mut target_amounts,
            config.max_key_group_size,
            config.max_target_group_size,
            config.n_candidates,
            false,
            false,
        ) {
            if !answers.is_empty() {
                // Find the answer with fewest remainders (i.e., most items matched)
                answers.sort_by_key(|a| a.keys_remainder.len() + a.targets_remainder.len());
                let best_answer = answers.remove(0);
                
                let mut key_pool: HashMap<i64, Vec<Transaction>> = HashMap::new();
                for k in remaining_keys.clone() {
                    key_pool.entry(round(k.amount)).or_default().push(k);
                }
                let mut target_pool: HashMap<i64, Vec<Transaction>> = HashMap::new();
                for t in remaining_targets.clone() {
                    target_pool.entry(round(t.amount)).or_default().push(t);
                }

                for (k_group, t_group) in best_answer.answer_arr {
                    let mut mg_keys = Vec::new();
                    let mut mg_targets = Vec::new();
                    
                    for k_val in &k_group {
                        if let Some(pool) = key_pool.get_mut(k_val) {
                            if let Some(k_txn) = pool.pop() {
                                mg_keys.push(k_txn.clone());
                                matched_key_ids.insert(k_txn.id.clone());
                            }
                        }
                    }
                    for t_val in &t_group {
                        if let Some(pool) = target_pool.get_mut(t_val) {
                            if let Some(t_txn) = pool.pop() {
                                mg_targets.push(t_txn.clone());
                                matched_target_ids.insert(t_txn.id.clone());
                            }
                        }
                    }
                    
                    let key_sum = mg_keys.iter().map(|k| k.amount).sum();
                    let target_sum = mg_targets.iter().map(|t| t.amount).sum();
                    
                    if !mg_keys.is_empty() || !mg_targets.is_empty() {
                        matched_groups.push(MatchedGroup {
                            keys: mg_keys,
                            targets: mg_targets,
                            key_sum,
                            target_sum,
                            difference: key_sum - target_sum,
                        });
                    }
                }
            }
        }
    }

    let final_unmatched_keys: Vec<Transaction> = keys
        .into_iter()
        .filter(|k| !matched_key_ids.contains(&k.id))
        .collect();
    let final_unmatched_targets: Vec<Transaction> = targets
        .into_iter()
        .filter(|t| !matched_target_ids.contains(&t.id))
        .collect();

    let matched_key_amount = matched_groups.iter().map(|g| g.key_sum).sum();
    let unmatched_key_amount = final_unmatched_keys.iter().map(|k| k.amount).sum();
    let unmatched_target_amount = final_unmatched_targets.iter().map(|t| t.amount).sum();

    let summary = ReconciliationSummary {
        total_keys: total_keys_count,
        total_targets: total_targets_count,
        matched_key_count: matched_key_ids.len(),
        matched_target_count: matched_target_ids.len(),
        matched_amount: matched_key_amount,
        unmatched_key_amount,
        unmatched_target_amount,
    };

    Ok(ReconciliationResult {
        matched: matched_groups,
        unmatched_keys: final_unmatched_keys,
        unmatched_targets: final_unmatched_targets,
        summary,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tx(id: &str, amount: i64) -> Transaction {
        Transaction {
            id: id.to_string(),
            amount,
            date: None,
            description: None,
        }
    }

    #[test]
    fn test_reconcile_exact_matches() {
        let keys = vec![
            tx("k1", 100),
            tx("k2", 200),
            tx("k3", 300),
        ];
        let targets = vec![
            tx("t1", 200),
            tx("t2", 100),
            tx("t3", 400),
        ];
        
        let config = ReconciliationConfig {
            max_key_group_size: 5,
            max_target_group_size: 5,
            tolerance: 0,
            n_candidates: 10,
        };

        let result = reconcile(keys, targets, config).unwrap();
        
        assert_eq!(result.matched.len(), 2);
        assert_eq!(result.unmatched_keys.len(), 1);
        assert_eq!(result.unmatched_keys[0].id, "k3");
        assert_eq!(result.unmatched_targets.len(), 1);
        assert_eq!(result.unmatched_targets[0].id, "t3");
    }

    #[test]
    fn test_reconcile_many_to_many() {
        let keys = vec![
            tx("k1", 100),
            tx("k2", 200),
            tx("k3", 500),
        ];
        let targets = vec![
            tx("t1", 300), // matches k1 + k2
            tx("t2", 200), // matches partial k3? No, 500 = 200 + 300
            tx("t3", 300),
        ];
        
        let config = ReconciliationConfig {
            max_key_group_size: 5,
            max_target_group_size: 5,
            tolerance: 0,
            n_candidates: 10,
        };

        let result = reconcile(keys, targets, config).unwrap();
        
        assert_eq!(result.unmatched_keys.len(), 0);
        assert_eq!(result.unmatched_targets.len(), 0);
        assert_eq!(result.summary.matched_amount, 800);
    }

    #[test]
    fn test_reconcile_with_tolerance() {
        let keys = vec![
            tx("k1", 103),
            tx("k2", 198),
        ];
        let targets = vec![
            tx("t1", 100),
            tx("t2", 200),
        ];
        
        let config = ReconciliationConfig {
            max_key_group_size: 5,
            max_target_group_size: 5,
            tolerance: 10,
            n_candidates: 10,
        };

        let result = reconcile(keys, targets, config).unwrap();
        assert_eq!(result.unmatched_keys.len(), 0);
        assert_eq!(result.unmatched_targets.len(), 0);
        assert_eq!(result.matched.len(), 2);
        
        let mut diffs: Vec<i64> = result.matched.iter().map(|m| m.difference).collect();
        diffs.sort();
        assert_eq!(diffs, vec![-2, 3]); 
    }
}