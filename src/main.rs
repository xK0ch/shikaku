mod game;
mod generator;
mod solver;
mod ui;

use leptos::prelude::*;

use crate::game::{Clue, Coord, Puzzle, Rect};
use crate::generator::generate;
use crate::ui::Board;

const SIZE_OPTIONS: &[usize] = &[5, 10, 20, 30, 40];
const DEFAULT_SIZE: usize = 5;

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

fn generator_params(size: usize) -> (usize, usize) {
    match size {
        0..=5 => (2, 500),
        6..=10 => (3, 1000),
        11..=20 => (5, 1500),
        21..=30 => (7, 2000),
        _ => (9, 3000),
    }
}

fn fresh_puzzle(size: usize) -> Puzzle {
    let (min_area, attempts) = generator_params(size);
    generate(size, size, min_area, attempts).unwrap_or_else(fallback_puzzle)
}

#[component]
fn App() -> impl IntoView {
    let current_size = RwSignal::new(DEFAULT_SIZE);
    let puzzle = RwSignal::new(fresh_puzzle(DEFAULT_SIZE));
    let placed = RwSignal::<Vec<Rect>>::new(vec![]);
    let drag_start = RwSignal::<Option<Coord>>::new(None);
    let drag_end = RwSignal::<Option<Coord>>::new(None);
    let message = RwSignal::<Option<String>>::new(None);

    let solved = move || {
        let clue_count = puzzle.with(|p| p.clues.len());
        let placed_count = placed.with(|ps| ps.len());
        clue_count > 0 && placed_count == clue_count
    };

    let reset_state = move || {
        placed.set(vec![]);
        drag_start.set(None);
        drag_end.set(None);
        message.set(None);
    };

    let new_puzzle = move |_| {
        puzzle.set(fresh_puzzle(current_size.get()));
        reset_state();
    };

    let reset = move |_| {
        reset_state();
    };

    let on_size_change = move |ev| {
        let Ok(size) = event_target_value(&ev).parse::<usize>() else { return };
        if size == current_size.get() {
            return;
        }
        current_size.set(size);
        puzzle.set(fresh_puzzle(size));
        reset_state();
    };

    view! {
        <main>
            <h1>"Shikaku"</h1>

            <section class="rules">
                <h2>"Regeln"</h2>
                <ol>
                    <li>"Jedes Rechteck enthält genau eine Zahl."</li>
                    <li>"Die Fläche des Rechtecks entspricht dieser Zahl."</li>
                    <li>"Rechtecke überlappen nicht und decken das ganze Brett ab."</li>
                </ol>
            </section>

            <div class="controls">
                <label class="size-picker">
                    "Größe: "
                    <select on:change=on_size_change>
                        {SIZE_OPTIONS.iter().map(|&s| {
                            view! {
                                <option
                                    value=s.to_string()
                                    selected=move || current_size.get() == s
                                >
                                    {format!("{s} \u{00d7} {s}")}
                                </option>
                            }
                        }).collect_view()}
                    </select>
                </label>
            </div>

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
