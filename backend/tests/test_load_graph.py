import yaml
import os
from osmnx_utils.load_graph import load_graph

def test_load_graph_yaml():
    with open("tests/graph_config_test.yaml", "r") as f:
        graph_config = yaml.safe_load(f)
            
    assert graph_config == {
        'map_dir_path': 'maps',
        'osmnx_graph_name': 'perth', 
        'network_type': 'drive', 
        'top_left_geo_coord': {'lat': -31.703589, 'long': 115.706794}, 
        'bottom_right_geo_coord': {'lat': -32.258136, 'long': 116.092168}
    }

def test_load_graph_file():
    load_graph("tests/graph_config_test.yaml")
    assert os.path.exists("maps/perth.json")