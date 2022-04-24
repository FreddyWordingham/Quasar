from fastapi import APIRouter, Request
import os
import re
import shutil
import signal

from . import component, settings


session_route = APIRouter()


class Session:
    """
    Session data.
    """

    data = {}


@session_route.post("/id/{session_id}/start")
async def start(session_id: str):
    """
    Create a new session.
    """

    # Remove illegal characters.
    session_id = re.sub("[^a-zA-Z\d:-_]", "", session_id)

    # Check session_id is not already in use.
    if session_id in Session.data.keys():
        raise ValueError(f"Session: '{session_id}' already exists.")

    Session.data[session_id] = init_session_filesystem(session_id)

    return session_id


@session_route.get("/id/{session_id}")
async def load(request: Request, session_id: str):
    """
    Load an existing session.
    """

    if session_id not in Session.data.keys():
        raise ValueError(f"Session: '{session_id}' does not exist.")

    active_plugins = [
        ["default", item_name]
        for item_name in await component.list_items("templates/plugins/default")
    ]

    return settings.TEMPLATES.TemplateResponse(
        "session.html",
        {
            "request": request,
            "session_id": session_id,
            "active_plugins": active_plugins,
        },
    )


@session_route.post("/id/{session_id}/end")
async def end(session_id: str):
    """
    End an existing session.
    """

    if session_id not in Session.data.keys():
        raise ValueError(f"Session: '{session_id}' does not exist.")

    Session.data.pop(session_id)

    return "Success"


@session_route.post("/clean")
async def clean():
    """
    Wipe all session data.
    """

    Session.data = {}

    for file in os.listdir(settings.SESSIONS_DIR):
        if file != "example":
            filepath = os.path.join(settings.SESSIONS_DIR, file)
            if os.path.isdir(filepath):
                shutil.rmtree(filepath)

    return "Success"


def init_session_filesystem(session_id: str):
    """
    Initialise the filesystem for a new session.
    Store path inforation in Session.data dictionary.
    """

    # Calculate file paths.
    dir = os.path.join(settings.SESSIONS_DIR, session_id)

    # Initialise files and directories.
    if os.path.exists(dir):
        shutil.rmtree(dir)
    os.makedirs(dir)

    return {"dir": dir}
