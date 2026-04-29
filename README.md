# Shikaku

A browser-playable [Shikaku](https://en.wikipedia.org/wiki/Shikaku) puzzle game written in Rust and compiled to WebAssembly with [Leptos](https://leptos.dev).

## About the game

Shikaku is a Japanese logic puzzle played on a rectangular grid. Some cells contain numbers. The goal is to divide the grid into rectangles such that:

1. Each rectangle contains exactly one numbered cell.
2. The area of the rectangle equals that number.
3. Rectangles do not overlap and cover the entire grid.

## Tech stack

- **Rust** for game logic, puzzle generator and solver
- **Leptos 0.7** (CSR mode) for the reactive UI
- **WebAssembly** as the browser target
- **Trunk** as the build tool and dev server

The application ships as a static bundle (HTML, JS glue, `.wasm`) and can be hosted on any static web server.

## Prerequisites

- Rust stable (1.95 or newer)
- The WebAssembly target:
  ```bash
  rustup target add wasm32-unknown-unknown
  ```
- Trunk:
  ```bash
  cargo install --locked trunk
  ```

## Running locally

From the project root:

```bash
trunk serve
```

Then open `http://localhost:8080` in your browser. Trunk watches the source files and rebuilds on every change.

## Building for production

```bash
trunk build --release
```

The output is written to `dist/` and can be deployed to any static host.
