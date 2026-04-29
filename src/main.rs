mod game;
mod generator;
mod i18n;
mod solver;
mod ui;

use leptos::prelude::*;

use crate::game::{Clue, Coord, Puzzle, Rect};
use crate::generator::generate;
use crate::i18n::Lang;
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
        0..=5 => (2, 10000),
        6..=10 => (3, 20000),
        11..=20 => (5, 30000),
        21..=30 => (7, 40000),
        _ => (9, 50000),
    }
}

fn try_fresh_puzzle(size: usize) -> Option<Puzzle> {
    let (min_area, attempts) = generator_params(size);
    generate(size, size, min_area, attempts)
}

fn fresh_puzzle(size: usize) -> Puzzle {
    try_fresh_puzzle(size).unwrap_or_else(fallback_puzzle)
}

#[component]
fn App() -> impl IntoView {
    let lang = RwSignal::new(Lang::De);
    provide_context(lang);

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

    let load_puzzle = move |size: usize| {
        match try_fresh_puzzle(size) {
            Some(p) => {
                puzzle.set(p);
                reset_state();
            }
            None => {
                let l = lang.get_untracked();
                message.set(Some(l.cannot_generate(size)));
            }
        }
    };

    let new_puzzle = move |_| {
        load_puzzle(current_size.get());
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
        load_puzzle(size);
    };

    let t = move || lang.get().texts();

    view! {
        <main lang=move || lang.get().code()>
            <header class="masthead">
                <span class="kanji" aria-hidden="true">"四角"</span>
                <h1>"Shikaku"</h1>
                <p class="tagline">
                    {move || t().tagline_prefix}
                    <span class="dot" aria-hidden="true"></span>
                    "Rust"
                    <span class="dot" aria-hidden="true"></span>
                    "WebAssembly"
                </p>
            </header>

            <div class="toolbar">
                <label class="size-picker">
                    {move || t().size_label}
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
                <div class="lang-switch" role="group" aria-label="Language">
                    <button
                        type="button"
                        class:active=move || lang.get() == Lang::De
                        on:click=move |_| lang.set(Lang::De)
                    >
                        "DE"
                    </button>
                    <button
                        type="button"
                        class:active=move || lang.get() == Lang::En
                        on:click=move |_| lang.set(Lang::En)
                    >
                        "EN"
                    </button>
                </div>
                <div class="actions">
                    <button class="btn" on:click=new_puzzle>{move || t().new_puzzle}</button>
                    <button class="btn" on:click=reset>{move || t().reset}</button>
                </div>
            </div>

            <div class="board-stage">
                <div class="board-frame">
                    <Board
                        puzzle=puzzle
                        placed=placed
                        drag_start=drag_start
                        drag_end=drag_end
                        message=message
                    />
                </div>
            </div>

            <div class="status">
                {move || {
                    if solved() {
                        Some(view! { <span class="success">{t().solved}</span> }.into_any())
                    } else {
                        message.get().map(|m| view! { <span class="error">{m}</span> }.into_any())
                    }
                }}
            </div>

            <section class="rules">
                <h2>{move || t().rules_heading}</h2>
                <ol>
                    <li>{move || t().rule_1}</li>
                    <li>{move || t().rule_2}</li>
                    <li>{move || t().rule_3}</li>
                </ol>
            </section>
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
