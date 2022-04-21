from fastapi import APIRouter, Request
import os

from . import settings


component_route = APIRouter()


@component_route.get("/list_items/{dir:path}")
async def list_items(dir):
    dir = os.path.join(settings.APP_DIR, dir)

    item_names = [os.path.splitext(filename)[0] for filename in os.listdir(dir)]
    return item_names
