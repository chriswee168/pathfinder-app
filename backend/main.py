from contextlib import asynccontextmanager
import os
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from osmnx_utils.load_graph import load_graph
import rust_lib

@asynccontextmanager
async def lifespan(app: FastAPI):
    graph_path, graph = load_graph("osmnx_utils/graph_config.yaml")
    test = rust_lib.create_graph(graph_path)
    test.display()
    yield

app = FastAPI(lifespan=lifespan)

# Frontend URL environment variable for hosting FastAPI on cloud service.
FRONTEND_URL = os.getenv("FRONTEND_URL", "http://127.0.0.1:5173")

# Origins to allow for Cross-Origin Resource Sharing in browser.
origins = [
    "http://localhost:5173", # Vite development.
    "http://127.0.0.1:5173",
    "http://localhost:4173", # Vite production preview.
    "http://127.0.0.1:4173",
    FRONTEND_URL
]

# Add CORS middleware.
app.add_middleware(
    CORSMiddleware,
    allow_origins=origins,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Check API status.
@app.get("/")
def get_api_status():
    return {"status": "Backend API online."}

# Add API routers.
#app.include_router()