use std::{cmp::Ordering, collections::HashMap};

use pyo3::pyclass;
use serde::Deserialize;

/// Struct for edge data in each Node.
#[pyclass]
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Edge { 
    pub index: Vec<u128>,
    pub street_name: String,
    pub speed_kph: f32,
    pub travel_time_s: f32,
    pub length_m: f32,
    pub highway: String,
    pub lat_long_coords: Vec<Vec<f32>>
}

/// Struct for Node data.
#[pyclass]
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Node { pub lat: f32, pub long: f32, pub edges: Vec<Edge> }

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