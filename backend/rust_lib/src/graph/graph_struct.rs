use std::{fs::File, io::BufReader};

use pyo3::prelude::*;

use crate::graph::types::GraphMap;

// Graph struct containing graph hashmap.
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
    pub fn new(mpk_path: String) -> Self
    {
        let graph_hashmap: GraphMap = load_from_mpk(mpk_path);
        return Graph { graph_hashmap };
    }

    pub fn display(&self)
    {
        println!("{:?}", self.graph_hashmap);
    }
}

/// Load map graph from MessagePack file.
fn load_from_mpk(mpk_path: String) -> GraphMap
{
    let file: File = File::open(mpk_path).unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let graph_hashmap: GraphMap = rmp_serde::from_read(reader).unwrap();

    return graph_hashmap;
}