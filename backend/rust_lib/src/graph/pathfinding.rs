use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::graph::{haversine::haversine, types::{GraphMap, Node, NodeIDWrapper}};

/// Implement A* search algorithm from start to end node.
/// Return the shortest route between start and end node as a vector of string tuples.
pub fn a_star_search(
    start_node_id: i64, 
    end_node_id: i64, 
    graph_hashmap: &GraphMap
) -> Option<Vec<(i64, i64)>>
{
    // Create essential heap and hashmap data structures.
    let mut node_id_priority_queue: BinaryHeap<NodeIDWrapper> = BinaryHeap::new();
    let mut cost_hashmap: HashMap<i64, (f32, i64)> = HashMap::new(); // node_id: (total_cost, prev_node_id)
    let mut visited_nodes_hashset: HashSet<i64> = HashSet::new();

    // Insert initial NodeIDWrapper.
    node_id_priority_queue.push(NodeIDWrapper {node_id: start_node_id, cost: 0.0});

    // Start node ID == End node ID.
    let mut destination_reached: bool = false;

    // Get end node's lat and long.
    let end_node: &Node = graph_hashmap.get(&end_node_id).unwrap();
    let end_node_lat: f32 = end_node.lat;
    let end_node_long: f32 = end_node.long;

    // A* search loop.
    while !node_id_priority_queue.is_empty() || !destination_reached
    {
        if let Some(current_node_id_wrapper) = node_id_priority_queue.pop()
        {
            let current_node_id: i64 = current_node_id_wrapper.node_id;
            let current_cost: f32 = current_node_id_wrapper.cost;

            // Terminate loop when reached destination.
            if current_node_id == end_node_id
            {
                destination_reached = true;
            }
            else
            {
                // Mark current node as visited.
                visited_nodes_hashset.insert(current_node_id);

                // Get actual node reference in graph and its edges.
                let node: &Node = graph_hashmap.get(&current_node_id).unwrap();
                for edge in &node.edges
                {
                    let dst_node_id: i64 = edge.dst_node_id;
                    if visited_nodes_hashset.contains(&dst_node_id)
                    {
                        let dst_node: &Node = graph_hashmap.get(&dst_node_id).unwrap();
                        let dst_node_lat: f32 = dst_node.lat;
                        let dst_node_long: f32 = dst_node.long;

                        // Calculate haversine distance between dst node and end node.
                        let hav_dist: f32 = haversine(
                            dst_node_lat, dst_node_long, 
                            end_node_lat, end_node_long
                        );

                        // Calculate updated cost from heuristic.
                        // Cost sign is flipped to make binary heap prioritise smaller costs.
                        let updated_cost: f32 = current_cost + edge.travel_time_s + hav_dist;

                        // Add dst node ID wrapper to priority queue.
                        let dst_node_id_wrapper: NodeIDWrapper = NodeIDWrapper {
                            node_id: dst_node_id, cost: updated_cost
                        };
                        node_id_priority_queue.push(dst_node_id_wrapper);

                        if let Some(dst_node_cost_details) = cost_hashmap.get_mut(&dst_node_id)
                        {
                            // Update dst node's cost and previous node if lower cost discovered.
                            if updated_cost < dst_node_cost_details.0
                            {
                                *dst_node_cost_details = (updated_cost, current_node_id);
                            }
                        }
                        else
                        {   // Add dst node details to cost hashmap if nonexistent.
                            cost_hashmap.insert(dst_node_id, (updated_cost, current_node_id));
                        }
                    }
                }
            }
        }
    }

    // Check if end node was reached, otherwise return None.
    if cost_hashmap.contains_key(&end_node_id)
    {
        // Get the shortest route from start to end node using cost hashmap.
        let mut shortest_route: Vec<(i64, i64)> = Vec::new();
        let mut current_id: Option<i64> = Some(end_node_id);
        while current_id.is_some()
        {
            // Get previous node ID using current node ID.
            let dst_node_id: i64 = current_id.unwrap();
            let src_node_id: i64 = cost_hashmap.get(&dst_node_id).unwrap().1;

            // Add TO and FROM node ID pair to shortest route vector.
            shortest_route.push((src_node_id, dst_node_id));
            
            if cost_hashmap.contains_key(&src_node_id)
            {
                current_id = Some(src_node_id);
            }
            else
            {
                // Starting node ID has been reached.
                // Node IDs don't have an entry in cost hashmap if it does not have an
                // origin node.
                current_id = None;
            }
        }

        // Reverse the order of shortest route to get correct order.
        shortest_route.reverse();

        return Some(shortest_route);
    }
    else
    {
        return None
    }
}