use std::{cmp::Ordering, collections::HashMap};

use serde::Deserialize;

/// Struct for edge data in each Node.
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Edge { 
    pub src_node_id: i64,
    pub dst_node_id: i64,
    pub street_name: String,
    pub speed_kph: f32,
    pub travel_time_s: f32,
    pub length_m: f32,
    pub highway: String,
    pub lat_long_coords: Vec<Vec<f32>>
}

/// Struct for Node data.
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Node { pub lat: f32, pub long: f32, pub edges: Vec<Edge> }

/// Final graph hash map type.
pub type GraphMap = HashMap<i64, Node>;

/// Wrapper struct for node ID priority queue used for A* search.
#[allow(dead_code)]
pub struct NodeIDWrapper {
    pub node_id: i64,
    pub cost: f32
}
impl PartialOrd for NodeIDWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}
impl PartialEq for NodeIDWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}
impl Ord for NodeIDWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.total_cmp(&self.cost)
    }
}
impl Eq for NodeIDWrapper {}