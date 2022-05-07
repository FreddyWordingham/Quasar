from fastapi import FastAPI, Request
from fastapi.responses import HTMLResponse
from fastapi.staticfiles import StaticFiles
import os

from . import settings
from .component import component_route
from .session import Session, session_route


app = FastAPI()
app.mount("/static", StaticFiles(directory="app/static"), name="static")
app.include_router(component_route, prefix="/component")
app.include_router(session_route, prefix="/session")


@app.get("/", response_class=HTMLResponse)
async def homepage(request: Request):
    """
    Session control.
    """

    return settings.TEMPLATES.TemplateResponse(
        "index.html", {"request": request, "sessions": Session.data.keys()}
    )


@app.get("/example_input")
async def example_input():
    """
    Get example input string.
    """

    return open(os.path.join(settings.SESSIONS_DIR, "example", "render.json")).read()
