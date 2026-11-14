use std::collections::{BinaryHeap, HashMap};

use crate::graph::types::{GraphMap, NodeIDWrapper};

/// Implement A* search algorithm from start to end node.
pub fn a_star_search(
    start_node: &str, 
    end_node: &str, 
    graph_hashmap: &GraphMap
)
{
    // Create essential heap and hashmap data structures.
    let node_id_priority_queue: BinaryHeap<NodeIDWrapper> = BinaryHeap::new();
    let cost_hashmap: HashMap<&str, (f32, &str)> = HashMap::new(); // node_id: (total_cost, prev_node_id)
    let visited_nodes_hashmap: HashMap<&str, bool> = HashMap::new();
}