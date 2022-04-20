from fastapi import FastAPI, Request
from fastapi.responses import HTMLResponse
from fastapi.staticfiles import StaticFiles

from . import settings

app = FastAPI()
app.mount("/static", StaticFiles(directory="app/static"), name="static")


class Session:
    data = {}


@app.get("/", response_class=HTMLResponse)
async def splashpage(request: Request):
    return settings.TEMPLATES.TemplateResponse(
        "index.html", {"request": request, "sessions": Session.data.keys()}
    )
