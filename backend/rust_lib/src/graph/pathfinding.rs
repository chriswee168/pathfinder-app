use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::graph::{haversine::haversine, types::{GraphMap, Node, NodeIDWrapper}};

/// Implement A* search algorithm from start to end node.
pub fn a_star_search(
    start_node_id: &str, 
    end_node_id: &str, 
    graph_hashmap: &GraphMap
)
{
    // Create essential heap and hashmap data structures.
    let mut node_id_priority_queue: BinaryHeap<NodeIDWrapper> = BinaryHeap::new();
    let mut cost_hashmap: HashMap<String, (f32, String)> = HashMap::new(); // node_id: (total_cost, prev_node_id)
    let mut visited_nodes_hashset: HashSet<String> = HashSet::new();

    // Insert initial NodeIDWrapper.
    node_id_priority_queue.push(NodeIDWrapper {node_id: start_node_id.to_string(), cost: 0.0});

    // Start node ID == End node ID.
    let mut destination_reached: bool = false;

    // Get end node's lat and long.
    let end_node: &Node = graph_hashmap.get(end_node_id).unwrap();
    let end_node_lat: f32 = end_node.lat;
    let end_node_long: f32 = end_node.long;

    // A* search loop.
    while !node_id_priority_queue.is_empty() || !destination_reached
    {
        if let Some(current_node_id_wrapper) = node_id_priority_queue.pop()
        {
            let current_node_id: String = current_node_id_wrapper.node_id;
            let current_cost: f32 = current_node_id_wrapper.cost;

            // Terminate loop when reached destination.
            if current_node_id == end_node_id
            {
                destination_reached = true;
            }
            else
            {
                // Mark current node as visited.
                visited_nodes_hashset.insert(current_node_id.clone());

                // Get actual node reference in graph and its edges.
                let node: &Node = graph_hashmap.get(&current_node_id).unwrap();
                for edge in &node.edges
                {
                    let next_node_id: String = edge.index[1].to_string();
                    if visited_nodes_hashset.contains(&next_node_id)
                    {
                        let next_node: &Node = graph_hashmap.get(&next_node_id).unwrap();
                        let next_node_lat: f32 = next_node.lat;
                        let next_node_long: f32 = next_node.long;

                        // Calculate haversine distance between next node and end node.
                        let hav_dist: f32 = haversine(
                            next_node_lat, next_node_long, 
                            end_node_lat, end_node_long
                        );

                        // Calculate updated cost from heuristic.
                        // Cost sign is flipped to make binary heap prioritise smaller costs.
                        let updated_cost: f32 = -(-current_cost + edge.travel_time_s + hav_dist);

                        // Add next node ID wrapper to priority queue.
                        let next_node_id_wrapper: NodeIDWrapper = NodeIDWrapper {
                            node_id: next_node_id.clone(), cost: updated_cost
                        };
                        node_id_priority_queue.push(next_node_id_wrapper);

                        if let Some(next_node_cost_details) = cost_hashmap.get_mut(&next_node_id)
                        {
                            // Update next node's cost and previous node if lower cost discovered.
                            if updated_cost < next_node_cost_details.0
                            {
                                *next_node_cost_details = (updated_cost, current_node_id.clone());
                            }
                        }
                        else
                        {   // Add next node details to cost hashmap if nonexistent.
                            cost_hashmap.insert(next_node_id, (updated_cost, current_node_id.clone()));
                        }
                    }
                }
            }
        }
    }
}