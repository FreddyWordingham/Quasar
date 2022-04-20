from fastapi import FastAPI, Request
from fastapi.responses import HTMLResponse
from fastapi.staticfiles import StaticFiles

from . import settings
from .component import component_route
from .hmi import hmi_route, Session
from .runner import runner_route, Session


app = FastAPI()
app.mount("/static", StaticFiles(directory="app/static"), name="static")
app.include_router(component_route, prefix="/component")
app.include_router(hmi_route, prefix="/hmi")
app.include_router(runner_route, prefix="/runner")


@app.get("/", response_class=HTMLResponse)
async def splashpage(request: Request):
    return settings.templates.TemplateResponse(
        "index.html", {"request": request, "sessions": Session.data.keys()}
    )
