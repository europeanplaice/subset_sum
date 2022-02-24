use pyo3::prelude::*;

/// Finds subsets sum of a target value. It can accept negative values.
#[pyfunction]
#[pyo3(text_signature = "(arr, value, /)")]
fn find_subset(arr: Vec<i32>, value: i32) -> PyResult<Vec<Vec<i32>>> {
    use crate::dp_module::*;
    Ok(dp::find_subset(&arr, value))
}

/// Finds subsets sum of a target value. It can't accept negative values but relatively faster.
#[pyfunction]
#[pyo3(text_signature = "(arr, value, /)")]
fn find_subset_fast_only_positive(arr: Vec<u32>, value: usize) -> PyResult<Vec<Vec<u32>>> {
    use crate::dp_module::*;
    Ok(dp::find_subset_fast_only_positive(&arr, value))
}

/// Finds the integers from two vectors that sum to the same value.
/// This method assumes that the two vectors have One-to-Many relationships.
/// Each integer of the `key` vector corresponds to the multiple integers of the `targets` vector.
#[pyfunction]
#[pyo3(text_signature = "(key, targets, /)")]
fn sequence_matcher(mut key: Vec<i32>, mut targets: Vec<i32>) -> PyResult<Vec<Vec<(Vec<i32>, i32)>>> {
    use crate::dp_module::*;
    Ok(dp::sequence_matcher(&mut key, &mut targets))
}

/// Finds the integers from two vectors that sum to the same value.
/// This method assumes that the two vectors have Many-to-Many relationships.
/// Each integer of the `arr1` vector corresponds to the multiple integers of the `arr2` vector.
/// With this method, we can find multiple combinations of the integers.
#[pyfunction]
#[pyo3(text_signature = "(arr1, arr2, /)")]
fn sequence_matcher_m2m(mut arr1: Vec<i32>, mut arr2: Vec<i32>) -> PyResult<Vec<Vec<(Vec<i32>, Vec<i32>)>>> {
    use crate::dp_module::*;
    Ok(dp::sequence_matcher_m2m(&mut arr1, &mut arr2, 10))
}

#[pymodule]
fn dpss(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(find_subset, m)?)?;
    m.add_function(wrap_pyfunction!(sequence_matcher, m)?)?;
    m.add_function(wrap_pyfunction!(sequence_matcher_m2m, m)?)?;
    m.add_function(wrap_pyfunction!(find_subset_fast_only_positive, m)?)?;

    Ok(())
}