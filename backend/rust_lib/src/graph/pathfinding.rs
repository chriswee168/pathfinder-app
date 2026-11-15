use std::collections::{BinaryHeap, HashMap};

use crate::graph::types::{GraphMap, Node, NodeIDWrapper};

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
    node_id_priority_queue.push(NodeIDWrapper {node_id: start_node_id.to_string(), cost: 0.0});

    // Start node ID == End node ID.
    let destination_reached: bool = false;

    // A* search loop.
    while !node_id_priority_queue.is_empty() || !destination_reached
    {
        if let Some(current_node_id_wrapper) = node_id_priority_queue.pop()
        {
            let current_node_id: String = current_node_id_wrapper.node_id;
            let current_cost: f32 = current_node_id_wrapper.cost;

            // Get actual node reference in graph and its edges.
            let node: &Node = graph_hashmap.get(&current_node_id).unwrap();
            for edge in &node.edges
            {
                let next_node_id: String = edge.index[1].to_string();

                // Calculate updated cost from heuristic.
                let updated_cost: f32 = current_cost + edge.travel_time_s;

                // Add next node ID wrapper to priority queue.
                let next_node_id_wrapper: NodeIDWrapper = NodeIDWrapper {
                    node_id: next_node_id, cost: updated_cost
                };
                node_id_priority_queue.push(next_node_id_wrapper);
            }
        }
    }
}