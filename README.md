# Quasar

![sphere](/examples/output/sphere.png)

A multi-threaded rendering engine written in Rust, and an optional FastAPI web app.

![app](/examples/output/app.png)

## Quickstart
```command
git clone https://github.com/FreddyWordingham/Quasar
cd Quasar
cargo build --release
poetry install
```

## Modes
![colour](/examples/output/colour.png)

Use the (pure Rust) program directly to render the image, and then combine the tiles with a python script:
```command
cargo run --bin render --release app/static/sessions example/render.json
python script/stitch.py app/static/sessions/example/tiles
```

![test](/examples/output/test.png)

Or spin up the application and watch the results stream in:
```command
poetry run uvicorn app.api.main:app
```