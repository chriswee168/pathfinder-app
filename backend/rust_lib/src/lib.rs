use pyo3::prelude::*;
mod graph;

/// A Python module implemented in Rust.
#[pymodule]
mod rust_lib 
{
    use pyo3::prelude::*;

    use crate::graph::{graph_struct::Graph};
    
    /// Function to create an return a new graph object.
    #[pyfunction]
    fn create_graph(graph_path: String) -> Graph
    {
        let graph: Graph = Graph::new(graph_path);
        return graph;
    }
}
