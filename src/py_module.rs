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
    Ok(dp::find_subset(&arr, value, max_length))
}

/// Finds subsets sum of a target value. It can't accept negative values but relatively faster.
/// # Arguments
/// * `arr` - An array.
/// * `value` - The value to the sum of the subset comes.
/// * `max_length` - The maximum length of combinations of the answer.
#[pyfunction]
#[pyo3(text_signature = "(arr, value, max_length, /)")]
fn find_subset_fast_only_positive(arr: Vec<u32>, value: usize, max_length: usize) -> PyResult<Vec<Vec<u32>>> {
    use crate::dp_module::*;
    Ok(dp::find_subset_fast_only_positive(&arr, value, max_length))
}

/// Finds the integers from two vectors that sum to the same value.
/// This method assumes that the two vectors have One-to-Many relationships.
/// Each integer of the `key` vector corresponds to the multiple integers of the `targets` vector.
/// # Arguments
/// * `key` - An array.
/// * `targets` - An array.
/// * `max_length` - The maximum length of combinations of the answer.
#[pyfunction]
#[pyo3(text_signature = "(key, targets, max_target_length, /)")]
fn sequence_matcher(mut key: Vec<i32>, mut targets: Vec<i32>, max_target_length: usize) -> PyResult<Vec<Vec<(i32, Vec<i32>)>>> {
    use crate::dp_module::*;
    Ok(dp::sequence_matcher(&mut key, &mut targets, max_target_length))
}

/// Finds the integers from two vectors that sum to the same value.
/// This method assumes that the two vectors have Many-to-Many relationships.
/// Each integer of the `keys` vector corresponds to the multiple integers of the `targets` vector.
/// With this method, we can find multiple combinations of the integers.
/// `n_candidates` is the number of candidates to be selected.
/// `max_key_length` is the maximum length of the keys as a group.
/// Especially in long sequences, this method is very slow so `n_candidates` and `max_key_length` should be small.
/// # Arguments
/// * `keys` - An array.
/// * `targets` - An array.
/// * `n_candidates` - The maximum length of combinations of the answer.
/// * `max_key_length` - The maximum length of combinations of the keys.
/// * `max_target_length` - The maximum length of combinations of the targets.
#[pyfunction]
#[pyo3(text_signature = "(keys, targets, n_candidates, max_key_length, max_target_length, /)")]
fn sequence_matcher_m2m(mut keys: Vec<i32>, mut targets: Vec<i32>, n_candidates: usize, max_key_length: usize, max_target_length: usize) -> PyResult<Vec<Vec<(Vec<i32>, Vec<i32>)>>> {
    use crate::dp_module::*;
    Ok(dp::sequence_matcher_m2m(&mut keys, &mut targets, n_candidates, max_key_length, max_target_length))
}

#[pymodule]
fn dpss(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(find_subset, m)?)?;
    m.add_function(wrap_pyfunction!(sequence_matcher, m)?)?;
    m.add_function(wrap_pyfunction!(sequence_matcher_m2m, m)?)?;
    m.add_function(wrap_pyfunction!(find_subset_fast_only_positive, m)?)?;

    Ok(())
}