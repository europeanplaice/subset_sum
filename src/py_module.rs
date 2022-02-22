use pyo3::prelude::*;

/// Finds subsets sum of a target value. It can accept negative values.
#[pyfunction]
#[pyo3(text_signature = "(a, n, /)")]
fn find_subset(a: Vec<i32>, n: i32) -> PyResult<Vec<Vec<i32>>> {
    use crate::dp_module::*;
    Ok(dp::find_subset(&a, n))
}

/// Finds subsets sum of a target value. It can't accept negative values but relatively faster.
#[pyfunction]
#[pyo3(text_signature = "(a, n, /)")]
fn find_subset_fast_only_positive(a: Vec<u32>, n: usize) -> PyResult<Vec<Vec<u32>>> {
    use crate::dp_module::*;
    Ok(dp::find_subset_fast_only_positive(&a, n))
}

/// Finds the integers from two vectors that sum to the same value.
/// This method assumes that the two vectors have One-to-Many relationships.
/// Each integer of the `key` vector corresponds to the multiple integers of the `value` vector.
#[pyfunction]
#[pyo3(text_signature = "(key, targets, /)")]
fn sequence_matcher(mut key: Vec<i32>, mut targets: Vec<i32>) -> PyResult<Vec<Vec<(Vec<i32>, i32)>>> {
    use crate::dp_module::*;
    Ok(dp::sequence_matcher(&mut key, &mut targets))
}

/// Finds the integers from two vectors that sum to the same value.
/// This method assumes that the two vectors have Many-to-Many relationships.
/// Each integer of the `key` vector corresponds to the multiple integers of the `value` vector.
/// With this method, we can find multiple combinations of the integers.
#[pyfunction]
#[pyo3(text_signature = "(key, targets, /)")]
fn sequence_matcher_m2m(mut key: Vec<i32>, mut targets: Vec<i32>) -> PyResult<Vec<Vec<(Vec<i32>, Vec<i32>)>>> {
    use crate::dp_module::*;
    Ok(dp::sequence_matcher_m2m(&mut key, &mut targets, 10))
}

#[pymodule]
fn dpss(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(find_subset, m)?)?;
    m.add_function(wrap_pyfunction!(sequence_matcher, m)?)?;
    m.add_function(wrap_pyfunction!(sequence_matcher_m2m, m)?)?;
    m.add_function(wrap_pyfunction!(find_subset_fast_only_positive, m)?)?;

    Ok(())
}