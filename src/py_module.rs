use pyo3::prelude::*;

#[pyfunction]
fn find_subset(a: Vec<i32>, n: i32) -> PyResult<Vec<Vec<i32>>> {
    use crate::dp_module::*;
    Ok(dp::find_subset(&a, n))
}

#[pyfunction]
fn find_subset_fast_only_positive(a: Vec<u32>, n: usize) -> PyResult<Vec<Vec<u32>>> {
    use crate::dp_module::*;
    Ok(dp::find_subset_fast_only_positive(&a, n))
}

#[pyfunction]
fn sequence_matcher(mut key: Vec<i32>, mut targets: Vec<i32>) -> PyResult<Vec<Vec<(Vec<i32>, i32)>>> {
    use crate::dp_module::*;
    Ok(dp::sequence_matcher(&mut key, &mut targets))
}

#[pyfunction]
fn sequence_matcher_m2m(mut key: Vec<i32>, mut targets: Vec<i32>) -> PyResult<Vec<Vec<(Vec<i32>, i32)>>> {
    use crate::dp_module::*;
    Ok(dp::sequence_matcher(&mut key, &mut targets))
}

#[pymodule]
fn dpss(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(find_subset, m)?)?;
    m.add_function(wrap_pyfunction!(sequence_matcher, m)?)?;
    m.add_function(wrap_pyfunction!(sequence_matcher_m2m, m)?)?;
    m.add_function(wrap_pyfunction!(find_subset_fast_only_positive, m)?)?;

    Ok(())
}