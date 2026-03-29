use crate::reconciliation;
use pyo3::prelude::*;

/// Finds subsets sum of a target value. It can accept negative values.
/// # Arguments
/// * `arr` - An array.
/// * `value` - The value to the sum of the subset comes.
/// * `max_length` - The maximum length of combinations of the answer.
#[pyfunction]
#[pyo3(text_signature = "(arr, value, max_length, /)")]
fn find_subset(arr: Vec<i64>, value: i64, max_length: usize) -> PyResult<Vec<Vec<i64>>> {
    use crate::dp_module::*;
    Ok(dp::find_subset(arr, value, max_length))
}

/// Finds the integers from two vectors that sum to the same value.
/// This method assumes that the two vectors have Many-to-Many relationships.
/// Each integer of the `keys` vector corresponds to the multiple integers of the `targets` vector.
/// With this method, we can find combinations of the integers.
/// To avoid combinatorial explosion, some parameters need to be set.
/// `max_key_length` is used to restrict the number of values in keys chosen.
/// If `max_key_length` is 3, an answer's length is at most 3, such as `[1980 + 2980 + 3500], [1050]`
/// `max_target_length` is the same as `max_key_length` for targets.
/// `n_candidates` specifies the maximum number of pattern.
/// If `use_all_keys` is true, an answer must contain all the elements of the keys.
/// If `use_all_targets` is true, an answer must contain all the elements of the targets.
/// When both `use_all_keys` and `use_all_targets` are true, the sum of the keys and the targets must be the same.
/// # Arguments
/// * `keys` - An array.
/// * `targets` - An array.
/// * `max_key_length` - An integer.
/// * `max_target_length` - An integer.
/// * `n_candidates` - An integer.
/// * `use_all_keys` - Boolean.
/// * `use_all_targets` - Boolean.
#[pyfunction]
#[pyo3(
    text_signature = "(keys, targets, max_key_length, max_target_length, n_candidates, use_all_keys, use_all_targets /)"
)]
fn sequence_matcher(
    mut keys: Vec<i64>,
    mut targets: Vec<i64>,
    max_key_length: usize,
    max_target_length: usize,
    n_candidates: usize,
    use_all_keys: bool,
    use_all_targets: bool,
) -> PyResult<Vec<(Vec<(Vec<i64>, Vec<i64>)>, Vec<i64>, Vec<i64>)>> {
    use crate::dp_module::*;
    use pyo3::exceptions::PyValueError;
    match dp::sequence_matcher(
        &mut keys,
        &mut targets,
        max_key_length,
        max_target_length,
        n_candidates,
        use_all_keys,
        use_all_targets,
    ) {
        Ok(res) => {
            let mut v = vec![];
            for r in res {
                v.push((r.answer_arr, r.keys_remainder, r.targets_remainder))
            }
            Ok(v)
        }
        Err(error) => Err(PyValueError::new_err(error)),
    }
}

#[pyfunction]
fn sequence_matcher_formatter(
    result: Vec<(Vec<(Vec<i64>, Vec<i64>)>, Vec<i64>, Vec<i64>)>,
) -> PyResult<String> {
    use crate::dp_module::*;
    let mut v = vec![];
    for r in result {
        v.push(dp::AnswerElement {
            answer_arr: r.0,
            keys_remainder: r.1,
            targets_remainder: r.2,
        })
    }
    Ok(dp::sequence_matcher_formatter(v))
}

#[pyclass(get_all)]
#[derive(Clone)]
pub struct Transaction {
    pub id: String,
    pub amount: i64,
    pub date: Option<String>,
    pub description: Option<String>,
}

#[pymethods]
impl Transaction {
    #[new]
    #[pyo3(signature = (id, amount, date=None, description=None))]
    fn new(id: String, amount: i64, date: Option<String>, description: Option<String>) -> Self {
        Transaction {
            id,
            amount,
            date,
            description,
        }
    }
    fn __repr__(&self) -> String {
        format!("Transaction(id='{}', amount={})", self.id, self.amount)
    }
}

impl From<Transaction> for reconciliation::Transaction {
    fn from(py_tx: Transaction) -> Self {
        reconciliation::Transaction {
            id: py_tx.id,
            amount: py_tx.amount,
            date: py_tx.date,
            description: py_tx.description,
        }
    }
}

impl From<reconciliation::Transaction> for Transaction {
    fn from(tx: reconciliation::Transaction) -> Self {
        Transaction {
            id: tx.id,
            amount: tx.amount,
            date: tx.date,
            description: tx.description,
        }
    }
}

#[pyclass(get_all)]
#[derive(Clone)]
pub struct MatchedGroup {
    pub keys: Vec<Transaction>,
    pub targets: Vec<Transaction>,
    pub key_sum: i64,
    pub target_sum: i64,
    pub difference: i64,
}

#[pymethods]
impl MatchedGroup {
    fn __repr__(&self) -> String {
        format!(
            "MatchedGroup(keys=[{}], targets=[{}], diff={})",
            self.keys.len(),
            self.targets.len(),
            self.difference
        )
    }
}

impl From<reconciliation::MatchedGroup> for MatchedGroup {
    fn from(mg: reconciliation::MatchedGroup) -> Self {
        MatchedGroup {
            keys: mg.keys.into_iter().map(Transaction::from).collect(),
            targets: mg.targets.into_iter().map(Transaction::from).collect(),
            key_sum: mg.key_sum,
            target_sum: mg.target_sum,
            difference: mg.difference,
        }
    }
}

#[pyclass(get_all)]
#[derive(Clone)]
pub struct ReconciliationSummary {
    pub total_keys: usize,
    pub total_targets: usize,
    pub matched_key_count: usize,
    pub matched_target_count: usize,
    pub matched_amount: i64,
    pub unmatched_key_amount: i64,
    pub unmatched_target_amount: i64,
}

#[pymethods]
impl ReconciliationSummary {
    fn __repr__(&self) -> String {
        format!(
            "ReconciliationSummary(keys_matched={}/{}, targets_matched={}/{})",
            self.matched_key_count, self.total_keys, self.matched_target_count, self.total_targets
        )
    }
}

impl From<reconciliation::ReconciliationSummary> for ReconciliationSummary {
    fn from(s: reconciliation::ReconciliationSummary) -> Self {
        ReconciliationSummary {
            total_keys: s.total_keys,
            total_targets: s.total_targets,
            matched_key_count: s.matched_key_count,
            matched_target_count: s.matched_target_count,
            matched_amount: s.matched_amount,
            unmatched_key_amount: s.unmatched_key_amount,
            unmatched_target_amount: s.unmatched_target_amount,
        }
    }
}

#[pyclass(get_all)]
#[derive(Clone)]
pub struct ReconciliationResult {
    pub matched: Vec<MatchedGroup>,
    pub unmatched_keys: Vec<Transaction>,
    pub unmatched_targets: Vec<Transaction>,
    pub summary: ReconciliationSummary,
}

#[pymethods]
impl ReconciliationResult {
    fn __repr__(&self) -> String {
        format!(
            "ReconciliationResult(matched_groups={}, unmatched_keys={}, unmatched_targets={})",
            self.matched.len(),
            self.unmatched_keys.len(),
            self.unmatched_targets.len()
        )
    }
}

impl From<reconciliation::ReconciliationResult> for ReconciliationResult {
    fn from(r: reconciliation::ReconciliationResult) -> Self {
        ReconciliationResult {
            matched: r.matched.into_iter().map(MatchedGroup::from).collect(),
            unmatched_keys: r
                .unmatched_keys
                .into_iter()
                .map(Transaction::from)
                .collect(),
            unmatched_targets: r
                .unmatched_targets
                .into_iter()
                .map(Transaction::from)
                .collect(),
            summary: ReconciliationSummary::from(r.summary),
        }
    }
}

#[pyfunction]
#[pyo3(signature = (keys, targets, max_key_group_size, max_target_group_size, tolerance=0, n_candidates=10))]
fn reconcile(
    keys: Vec<Transaction>,
    targets: Vec<Transaction>,
    max_key_group_size: usize,
    max_target_group_size: usize,
    tolerance: i64,
    n_candidates: usize,
) -> PyResult<ReconciliationResult> {
    let rs_keys = keys
        .into_iter()
        .map(reconciliation::Transaction::from)
        .collect();
    let rs_targets = targets
        .into_iter()
        .map(reconciliation::Transaction::from)
        .collect();
    let config = reconciliation::ReconciliationConfig {
        max_key_group_size,
        max_target_group_size,
        tolerance,
        n_candidates,
    };
    match reconciliation::reconcile(rs_keys, rs_targets, config) {
        Ok(res) => Ok(ReconciliationResult::from(res)),
        Err(e) => Err(pyo3::exceptions::PyValueError::new_err(e)),
    }
}

#[pymodule]
fn dpss(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Transaction>()?;
    m.add_class::<MatchedGroup>()?;
    m.add_class::<ReconciliationSummary>()?;
    m.add_class::<ReconciliationResult>()?;
    m.add_function(wrap_pyfunction!(find_subset, m)?)?;
    m.add_function(wrap_pyfunction!(sequence_matcher, m)?)?;
    m.add_function(wrap_pyfunction!(sequence_matcher_formatter, m)?)?;
    m.add_function(wrap_pyfunction!(reconcile, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
