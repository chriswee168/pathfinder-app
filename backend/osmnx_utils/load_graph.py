from typing import Any
from networkx import MultiDiGraph
import osmnx as ox
import yaml
import os
import json

def load_graph(graph_config_path: str) -> dict:
    """
    Function to download a new OpenStreetMap graph or load an existing one.

    Parameters:
        graph_config_path (str): Path to YAML graph configuration file.
    
    Returns:
        graph_dict (dict): Graph dictionary.
    """

    with open(graph_config_path, "r") as f:
        graph_config = yaml.safe_load(f)

    map_dir_path: str = graph_config["map_dir_path"]
    graph_name: str = graph_config["osmnx_graph_name"]
    network_type: str = graph_config["network_type"]
    top_lat: float = graph_config["top_left_geo_coord"]["lat"]
    left_long: float = graph_config["top_left_geo_coord"]["long"]
    bottom_lat: float = graph_config["bottom_right_geo_coord"]["lat"]
    right_long: float = graph_config["bottom_right_geo_coord"]["long"]

    graph_path = os.path.join(map_dir_path, f"{graph_name}.json")

    if not os.path.exists(map_dir_path):
        os.makedirs(map_dir_path)

    # Download graph and save as dictionary, or load existing graph dictionary from
    # JSON.
    if not os.path.exists(graph_path):
        print(
            f"Downloading graph between (lat: {top_lat}, long: {left_long}) and "
            f"(lat: {bottom_lat}, long: {right_long})..."
        )
        graph = ox.graph_from_bbox(
            (left_long, bottom_lat, right_long, top_lat), 
            network_type=network_type
        )

        graph_dict = create_graph_dict(graph)
        
        with open(graph_path, "w") as f:
            json.dump(graph_dict, f, indent=4)

        print(f"Saved graph to {graph_path}")

    else:
        print(f"Loading existing graph {graph_path}")

        with open(graph_path, "r") as f:
            graph_dict = json.load(f)
            
        print(f"Graph {graph_path} loaded.")

    return graph_path, graph_dict

def create_graph_dict(graph):
    """
    Create graph dictionary containing nodes and their edges from OSMnx graph.

    Parameters:
        graph (MultiDiGraph[Any, dict[str, Any], dict[str, Any]]): Graph object.
    
    Returns:
        graph_dict (dict): Graph dictionary containing nodes and their edges.
    """
    graph = ox.routing.add_edge_speeds(graph, fallback=25)
    graph = ox.routing.add_edge_travel_times(graph)

    nodes, edges = ox.graph_to_gdfs(graph)

    # Initialize the graph dictionary.
    graph_dict: dict = {}
    for index, row in nodes.iterrows():
        graph_dict.update({
            index: {
                "lat": row["y"],
                "long": row["x"],
                "edges": []
            }
        })

    selected_edges = edges["maxspeed"].notna() & edges["name"].notna()
    edges = edges[selected_edges]
    
    for index, row in edges.iterrows():
        u = index[0]
        v = index[1]
        key = index[2]
        street_name = row["name"]
        speed_kph = row["speed_kph"]
        travel_time_s = row["travel_time"]
        length_m = row["length"]
        highway = row["highway"]
        geometry = row["geometry"]
        lat_long_coords = [(lat, long) for long, lat in geometry.coords]

        # Convert lists to strings joined with forward slash.
        if isinstance(street_name, list):
            street_name = "/".join(street_name)
        if isinstance(highway, list):
            highway = "/".join(highway)
        
        graph_dict[u]["edges"].append({
            "index": (u, v, key), 
            "street_name": street_name,
            "speed_kph": speed_kph,
            "travel_time_s": travel_time_s,
            "length_m": length_m,
            "highway": highway,
            "lat_long_coords": lat_long_coords
        })

    return graph_dict