use pyo3::prelude::*;

/// Finds subsets sum of a target value. It can accept negative values.
/// # Arguments
/// * `arr` - An array.
/// * `value` - The value to the sum of the subset comes.
/// * `max_length` - The maximum length of combinations of the answer.
#[pyfunction]
#[pyo3(text_signature = "(arr, value, max_length, /)")]
fn find_subset(arr: Vec<i32>, value: i32, max_length: usize) -> PyResult<Vec<Vec<i32>>> {
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
#[pyo3(text_signature = "(keys, targets, max_key_length, max_target_length, n_candidates, use_all_keys, use_all_targets /)")]
fn sequence_matcher(
    mut keys: Vec<i32>,
    mut targets: Vec<i32>,
    max_key_length: usize,
    max_target_length: usize,
    n_candidates: usize,
    use_all_keys: bool,
    use_all_targets: bool,
) -> PyResult<Vec<Vec<(Vec<i32>, Vec<i32>)>>> {
    use crate::dp_module::*;
    use pyo3::exceptions::PyValueError;
    match dp::sequence_matcher(
        &mut keys,
        &mut targets,
        max_key_length,
        max_target_length,
        n_candidates,
        use_all_keys,
        use_all_targets
    ){
        Ok(res) => (Ok(res)),
        Err(error) => Err(PyValueError::new_err(error))
    }
}

#[pyfunction]
fn sequence_matcher_formatter(result: Vec<Vec<(Vec<i32>, Vec<i32>)>>) -> PyResult<String> {
    use crate::dp_module::*;
    Ok(dp::sequence_matcher_formatter(result))
}

#[pymodule]
fn dpss(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(find_subset, m)?)?;
    m.add_function(wrap_pyfunction!(sequence_matcher, m)?)?;
    m.add_function(wrap_pyfunction!(sequence_matcher_formatter, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}
