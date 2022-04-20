from fastapi import FastAPI, Request
from fastapi.responses import HTMLResponse

app = FastAPI()


@app.get("/", response_class=HTMLResponse)
async def splashpage(request: Request):
    return "<h1>Hello world!</h1>"
