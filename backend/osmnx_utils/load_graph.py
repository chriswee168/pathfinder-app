from typing import Any
from networkx import MultiDiGraph
import osmnx as ox
import yaml
import os

def load_graph(graph_config_path: str):
    """
    Function to download a new OpenStreetMap graph or load an existing one.

    Parameters:
        graph_config_path (str): Path to YAML graph configuration file.
    
    Returns:
        graph (MultiDiGraph[Any, dict[str, Any], dict[str, Any]]): Graph object.
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

    graph_path = os.path.join(map_dir_path, f"{graph_name}.graphml")

    if not os.path.exists(map_dir_path):
        os.makedirs(map_dir_path)
    
    if not os.path.exists(graph_path):
        print(
            f"Downloading graph between (lat: {top_lat}, long: {left_long}) and "
            f"(lat: {bottom_lat}, long: {right_long})..."
        )
        graph = ox.graph_from_bbox(
            (left_long, bottom_lat, right_long, top_lat), 
            network_type=network_type
        )
        ox.save_graphml(graph, graph_path)
        print(f"Saved graph to {graph_path}")

    else:
        print(f"Loading existing graph {graph_path}")
        graph = ox.load_graphml(graph_path)
        print(f"Graph {graph_path} loaded.")

    return graph