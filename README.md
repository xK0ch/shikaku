# Shikaku

A browser-playable [Shikaku](https://en.wikipedia.org/wiki/Shikaku) puzzle game written in Rust and compiled to WebAssembly with [Leptos](https://leptos.dev).

## About the game

Shikaku is a Japanese logic puzzle played on a rectangular grid. Some cells contain numbers. The goal is to divide the grid into rectangles such that:

1. Each rectangle contains exactly one numbered cell.
2. The area of the rectangle equals that number.
3. Rectangles do not overlap and cover the entire grid.

## Tech stack

- **Rust** for game logic, puzzle generator and solver
- **Leptos 0.8** (CSR mode) for the reactive UI
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

## Playing

Drag across the board with the mouse or a finger to draw a rectangle, tap or click a placed
rectangle to remove it. Pointer events handle mouse, touch and pen the same way, so the game works
on phones and tablets.

## Puzzle generator

The generator tiles the board with random rectangles and keeps the tiling only if the solver finds
exactly one solution. For each rectangle it first picks an area within the limits of the board size
(for example 3 to 15 on 10 x 10), slightly favouring smaller areas, and then a shape with that area.
This gives a mix of small, medium and a few large numbers instead of a handful of huge rectangles.

## Tests

```bash
cargo test
```

The Docker build runs the tests before the release build, so a failing test stops the deployment.

## Building for production

```bash
trunk build --release
```

The output is written to `dist/` and can be deployed to any static host.
