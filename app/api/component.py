from fastapi import APIRouter, Request
import os

from . import settings


component_route = APIRouter()


@component_route.get("/list_items/{dir:path}")
async def list_items(dir):
    """
    List the filenames in a given directory.
    """

    return [
        os.path.splitext(filename)[0]
        for filename in os.listdir(os.path.join(settings.APP_DIR, dir))
    ]
