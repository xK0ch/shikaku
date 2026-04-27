mod game;
mod generator;
mod solver;
mod ui;

use leptos::prelude::*;

use crate::game::{Clue, Coord, Puzzle, Rect};
use crate::generator::generate;
use crate::ui::Board;

const GRID_SIZE: usize = 5;
const GENERATOR_ATTEMPTS: usize = 500;

fn fallback_puzzle() -> Puzzle {
    Puzzle {
        rows: 4,
        cols: 4,
        clues: vec![
            Clue { at: (0, 0), value: 2 },
            Clue { at: (0, 1), value: 2 },
            Clue { at: (0, 3), value: 4 },
            Clue { at: (3, 0), value: 8 },
        ],
    }
}

fn fresh_puzzle() -> Puzzle {
    generate(GRID_SIZE, GRID_SIZE, GENERATOR_ATTEMPTS).unwrap_or_else(fallback_puzzle)
}

#[component]
fn App() -> impl IntoView {
    let puzzle = RwSignal::new(fresh_puzzle());
    let placed = RwSignal::<Vec<Rect>>::new(vec![]);
    let drag_start = RwSignal::<Option<Coord>>::new(None);
    let drag_end = RwSignal::<Option<Coord>>::new(None);
    let message = RwSignal::<Option<String>>::new(None);

    let solved = move || {
        let clue_count = puzzle.with(|p| p.clues.len());
        let placed_count = placed.with(|ps| ps.len());
        clue_count > 0 && placed_count == clue_count
    };

    let new_puzzle = move |_| {
        puzzle.set(fresh_puzzle());
        placed.set(vec![]);
        drag_start.set(None);
        drag_end.set(None);
        message.set(None);
    };

    let reset = move |_| {
        placed.set(vec![]);
        drag_start.set(None);
        drag_end.set(None);
        message.set(None);
    };

    view! {
        <main>
            <h1>"Shikaku"</h1>

            <Board
                puzzle=puzzle
                placed=placed
                drag_start=drag_start
                drag_end=drag_end
                message=message
            />

            <div class="status">
                {move || {
                    if solved() {
                        Some(view! { <span class="success">"Gelöst!"</span> }.into_any())
                    } else {
                        message.get().map(|m| view! { <span class="error">{m}</span> }.into_any())
                    }
                }}
            </div>

            <div class="actions">
                <button class="btn" on:click=new_puzzle>"Neues Puzzle"</button>
                <button class="btn" on:click=reset>"Reset"</button>
            </div>
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
