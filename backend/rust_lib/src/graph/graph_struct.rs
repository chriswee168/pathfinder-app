use std::{fs::File, io::BufReader};

use pyo3::prelude::*;

use crate::graph::types::GraphMap;

// Graph struct to 
#[pyclass]
pub struct Graph
{
    graph_hashmap: GraphMap
}
/// Methods for Graph struct.
#[pymethods]
impl Graph
{
    #[new]
    pub fn new(json_path: String) -> Self
    {
        let graph_hashmap: GraphMap = load_from_json(json_path);
        return Graph { graph_hashmap };
    }

    pub fn display(&self)
    {
        println!("{:?}", self.graph_hashmap);
    }
}

/// Load map graph from JSON file.
fn load_from_json(json_path: String) -> GraphMap
{
    let file: File = File::open(json_path).unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let graph_hashmap: GraphMap = serde_json::from_reader(reader).unwrap();

    return graph_hashmap;
}