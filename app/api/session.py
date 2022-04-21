from fastapi import APIRouter, Request
from fastapi.responses import HTMLResponse
import re

from . import settings, utility


session_route = APIRouter()


class Session:
    data = {}


@session_route.get("/{session_id}")
async def load(request: Request, session_id: str):
    """
    Load an existing session.
    """

    if session_id not in Session.data.keys():
        raise ValueError(f"Session: '{session_id}' does not exist.")

    active_plugins = [
        ["default", item_name] for item_name in utility.get_plugin_list("default")
    ]

    return settings.TEMPLATES.TemplateResponse(
        "session.html",
        {
            "request": request,
            "session_id": session_id,
            "active_plugins": active_plugins,
        },
    )


@session_route.post("/{session_id}/new")
async def new(session_id: str):
    """
    Create a new session.
    """

    session_id = re.sub("[^a-zA-Z\d:-_]", "", session_id)

    if session_id in Session.data.keys():
        raise ValueError(f"Session: '{session_id}' already exists.")

    Session.data[session_id] = {}

    return session_id


@session_route.post("/{session_id}/end")
async def end(session_id: str):
    """
    End an existing session.
    """

    if session_id not in Session.data.keys():
        raise ValueError(f"Session: '{session_id}' does not exist.")

    Session.data.pop(session_id)

    return "Success"
