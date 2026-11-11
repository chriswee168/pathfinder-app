from fastapi import APIRouter, status

router = APIRouter(prefix="/api/v1")

@router.get("/route", status_code=status.HTTP_200_OK)
def pathfind(
    start_lat: float,
    start_long: float,
    end_lat: float,
    end_long: float
):
    """
    Calculate map route between start and end geographical coordinates.

    Parameters:
        start_lat (float): Starting latitude.
        start_long (float): Starting longitude.
        end_lat (float): Ending latitude.
        end_long (float): Ending longitude.
    """
    