from fastapi import APIRouter, Request
import json
from pydantic import BaseModel
from subprocess import Popen
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

    if "process" in Session.data[session_id]:
        os.kill(Session.data[session_id]["process"].pid, signal.SIGTERM)

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


class Render(BaseModel):
    config: str


@session_route.post("/id/{session_id}/render")
async def render(session_id: str, config: Render):
    """
    Start a rendering simulation.
    """

    if session_id not in Session.data.keys():
        raise ValueError(f"Session: '{session_id}' does not exist.")

    config = json.loads(config.config)
    config["input_dir"] = "app/static/resources"
    config["output_dir"] = f"app/static/sessions/{session_id}"

    out_file = f"{settings.SESSIONS_DIR}/{session_id}/output.log"
    err_file = f"{settings.SESSIONS_DIR}/{session_id}/error.log"

    try:
        with open(
            os.path.join(Session.data[session_id]["dir"], "render.json"), "w"
        ) as file, open(out_file, "w") as out, open(err_file, "w") as err:
            file.write(json.dumps(config, indent=4))
            Session.data[session_id]["process"] = Popen(
                [
                    "cargo",
                    "run",
                    "--bin",
                    "render",
                    "--release",
                    f"{settings.SESSIONS_DIR}/{session_id}/render.json",
                ],
                stdout=out,
                stderr=err,
            )
    except Exception as e:
        return e

    return "Success"


@session_route.get("/id/{session_id}/render/progress")
async def render_progress(session_id: str):
    """
    Get the progress of the simulation.
    """

    import subprocess

    out_file = f"{settings.SESSIONS_DIR}/{session_id}/output.log"
    line = subprocess.check_output(["tail", "-1", out_file])

    return line


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
