use pyo3::prelude::*;

// Graph struct to 
#[pyclass]
pub struct Graph
{
    test_attr: f32
}

/// Methods for Graph struct.
#[pymethods]
impl Graph
{
    #[new]
    pub fn new(test_attr: f32) -> Self
    {
        return Graph
        {
            test_attr
        };
    }

    pub fn display(&self)
    {
        println!("{}", self.test_attr);
    }
}