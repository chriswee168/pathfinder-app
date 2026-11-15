use std::{cmp::Ordering, collections::HashMap};

use pyo3::pyclass;
use serde::Deserialize;

/// Struct for edge data in each Node.
#[pyclass]
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Edges { 
    index: Vec<u128>,
    street_name: String,
    speed_kph: f32,
    travel_time_s: f32,
    length_m: f32,
    highway: String,
    lat_long_coords: Vec<Vec<f32>>
}

/// Struct for Node data.
#[pyclass]
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Node { lat: f32, long: f32, edges: Vec<Edges> }

/// Final graph hash map type.
pub type GraphMap = HashMap<String, Node>;

/// Wrapper struct for node ID priority queue used for A* search.
pub struct NodeIDWrapper<'a> {
    pub node_id: &'a str,
    pub cost: f32
}
impl PartialOrd for NodeIDWrapper<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}
impl PartialEq for NodeIDWrapper<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}
impl Ord for NodeIDWrapper<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.total_cmp(&other.cost)
    }
}
impl Eq for NodeIDWrapper<'_> {}