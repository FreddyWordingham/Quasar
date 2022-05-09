# Quasar

A multi-threaded custom built rendering engine, using Rust for performance, and FastAPI for interfacing.

![app](/app/static/images/app.png)

## Quickstart
```command
git clone https://github.com/FreddyWordingham/Quasar
cd Quasar
cargo build --release
poetry install
```

## Modes

![colour](/app/static/images/colour.png)

Use the (pure Rust) program directly:
```command
cargo run --bin render --release app/static/sessions example/render.json
```

![test](/app/static/images/test.png)

Or spin up the application and watch the results stream in:
```command
poetry run uvicorn app.api.main:app
```