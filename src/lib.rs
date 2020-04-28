use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pyfunction;
use std::collections::HashMap;

#[pyfunction]
/// Formats the sum of two numbers as string.
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
/// Formats the sum of two numbers as string.
fn get_result() -> PyResult<HashMap<String, String>> {
    let mut result = HashMap::new();
    result.insert("name".to_string(), "kushal".to_string());
    result.insert("age".to_string(), "36".to_string());
    Ok(result)
}

#[pyfunction]
// Returns a Person class, takes a dict with {"name": "age", "age": 100} format.
fn give_me_a_person(data: &PyDict) -> PyResult<Person> {
    let name: String = data.get_item("name").unwrap().extract().unwrap();
    let age: i64 = data.get_item("age").unwrap().extract().unwrap();

    let p: Person = Person::new(name, age);
    Ok(p)
}

#[pyclass]
#[derive(Debug)]
struct Person {
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    age: i64,
}

#[pymethods]
impl Person {
    #[new]
    fn new(name: String, age: i64) -> Self {
        Person { name, age }
    }
}

#[pymodule]
/// A Python module implemented in Rust.
fn myfriendrust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(sum_as_string))?;
    m.add_wrapped(wrap_pyfunction!(get_result))?;
    m.add_wrapped(wrap_pyfunction!(give_me_a_person))?;
    m.add_class::<Person>()?;
    Ok(())
}
