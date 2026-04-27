mod game;
mod ui;

use leptos::prelude::*;

use crate::game::{Clue, Puzzle};
use crate::ui::Board;

fn sample_puzzle() -> Puzzle {
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

#[component]
fn App() -> impl IntoView {
    view! {
        <main>
            <h1>"Shikaku"</h1>
            <Board puzzle={sample_puzzle()} />
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
