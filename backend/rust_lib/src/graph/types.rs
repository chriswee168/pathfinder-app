use std::collections::HashMap;

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