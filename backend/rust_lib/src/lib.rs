use pyo3::prelude::*;
mod graph;

/// A Python module implemented in Rust.
#[pymodule]
mod rust_lib 
{
    use pyo3::prelude::*;

    use crate::graph::graph_struct::Graph;
    
    /// Function to create an return a new graph object.
    #[pyfunction]
    fn create_graph(test_attr: f32) -> Graph
    {
        let graph: Graph = Graph::new(test_attr);
        return graph;
    }
}
