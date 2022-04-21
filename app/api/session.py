from datetime import datetime
from fastapi import APIRouter, Request
from fastapi.responses import HTMLResponse
import os
import re
import shutil

from . import component, settings


session_route = APIRouter()


class Session:
    """
    Session data.
    """

    data = {}


@session_route.get("/clean")
async def clean():
    """
    Wipe all session data.
    """

    for file in os.listdir(settings.SESSIONS_DIR):
        filepath = os.path.join(settings.SESSIONS_DIR, file)
        if os.path.isdir(filepath):
            shutil.rmtree(filepath)

    return "Success"


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


@session_route.post("/id/{session_id}/new")
async def new(session_id: str):
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


@session_route.post("/id/{session_id}/end")
async def end(session_id: str):
    """
    End an existing session.
    """

    if session_id not in Session.data.keys():
        raise ValueError(f"Session: '{session_id}' does not exist.")

    Session.data.pop(session_id)

    return "Success"


def init_session_filesystem(session_id: str):
    """
    Check that the session_id is not already in use.
    Initialise the filesystem for a new session.
    Store path inforation in Session.data dictionary.
    """

    # Calculate file paths.
    dir = os.path.join(settings.SESSIONS_DIR, session_id)
    input = os.path.join(dir, "session.input")
    output = os.path.join(dir, "session.output")

    # Initialise files and directories.
    if os.path.exists(dir):
        shutil.rmtree(dir)
    os.makedirs(dir)

    creation_time = datetime.now().strftime("%Y-%m-%dT%H:%M:%S")
    with open(input, "w") as file:
        file.write(f"# SESSION INPUT\n# Start: {creation_time}\n")

    with open(output, "w") as file:
        file.write(f"# SESSION OUTPUT\n# Start: {creation_time}\n")

    {"dir": dir, "input": input, "output": output}
