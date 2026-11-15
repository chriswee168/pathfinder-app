use std::collections::{BinaryHeap, HashMap};

use crate::graph::types::{GraphMap, NodeIDWrapper};

/// Implement A* search algorithm from start to end node.
pub fn a_star_search(
    start_node_id: &str, 
    end_node_id: &str, 
    graph_hashmap: &GraphMap
)
{
    // Create essential heap and hashmap data structures.
    let mut node_id_priority_queue: BinaryHeap<NodeIDWrapper> = BinaryHeap::new();
    let cost_hashmap: HashMap<&str, (f32, &str)> = HashMap::new(); // node_id: (total_cost, prev_node_id)
    let visited_nodes_hashmap: HashMap<&str, bool> = HashMap::new();

    // Insert initial NodeIDWrapper.
    node_id_priority_queue.push(NodeIDWrapper {node_id: start_node_id, cost: 0.0});
}