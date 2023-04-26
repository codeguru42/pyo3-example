use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

fn main() {
    println!("Hello, world!");
}

#[pyclass]
#[derive(PartialEq, Debug)]
pub struct Example {
    num: i32,
}

#[pymethods]
impl Example {
    #[new]
    fn new(num: i32) -> PyResult<Self> {
        return if num > 0 {
            Ok(Self { num })
        } else {
            Err(PyValueError::new_err("num must be greater than 0"))
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let result = Example::new(42).unwrap();
        let expected = Example { num: 42 };
        assert_eq!(expected, result);
    }
}
