use leptos::prelude::*;

use crate::game::{Coord, PlacementError, Puzzle, Rect};

const PALETTE: &[&str] = &[
    "#fde2e2", "#dceefb", "#e2f5d8", "#fff1c2", "#ead4f5", "#d3f0ee", "#ffd9bd", "#e9e4d4",
];

#[component]
pub fn Board(puzzle: Puzzle) -> impl IntoView {
    let rows = puzzle.rows;
    let cols = puzzle.cols;

    let placed: RwSignal<Vec<Rect>> = RwSignal::new(vec![]);
    let drag_start: RwSignal<Option<Coord>> = RwSignal::new(None);
    let drag_end: RwSignal<Option<Coord>> = RwSignal::new(None);
    let message: RwSignal<Option<String>> = RwSignal::new(None);

    let pending_rect = move || match (drag_start.get(), drag_end.get()) {
        (Some(s), Some(e)) => Some(Rect::new(s.0, s.1, e.0, e.1)),
        _ => None,
    };

    let puzzle_for_commit = puzzle.clone();
    let commit_drag = move || {
        let Some(rect) = pending_rect() else {
            return;
        };
        let overlaps_existing = placed.with(|ps| ps.iter().any(|p| p.overlaps(&rect)));
        if overlaps_existing {
            message.set(Some("Überlappt mit einem bereits platzierten Rechteck.".into()));
        } else {
            match puzzle_for_commit.validate_placement(&rect) {
                Ok(_) => {
                    placed.update(|ps| ps.push(rect));
                    message.set(None);
                }
                Err(e) => message.set(Some(error_text(e))),
            }
        }
        drag_start.set(None);
        drag_end.set(None);
    };

    let cancel_drag = move || {
        drag_start.set(None);
        drag_end.set(None);
    };

    let clues = puzzle.clues.clone();
    let cells: Vec<_> = (0..rows)
        .flat_map(|r| {
            let clues = clues.clone();
            (0..cols).map(move |c| {
                let coord = (r, c);
                let value = clues.iter().find(|cl| cl.at == coord).map(|cl| cl.value);

                let style = move || {
                    if let Some(idx) = placed.with(|ps| ps.iter().position(|rect| rect.contains(coord))) {
                        return format!("background:{};", PALETTE[idx % PALETTE.len()]);
                    }
                    if let Some(p) = pending_rect() {
                        if p.contains(coord) {
                            return "background:rgba(110,170,240,0.35);".to_string();
                        }
                    }
                    String::new()
                };

                view! {
                    <div
                        class="cell"
                        style=style
                        on:mousedown=move |_| {
                            drag_start.set(Some(coord));
                            drag_end.set(Some(coord));
                            message.set(None);
                        }
                        on:mouseenter=move |_| {
                            if drag_start.get().is_some() {
                                drag_end.set(Some(coord));
                            }
                        }
                    >
                        {value.map(|v| v.to_string())}
                    </div>
                }
            })
        })
        .collect();

    let grid_style = format!(
        "grid-template-columns: repeat({cols}, var(--cell-size)); \
         grid-template-rows: repeat({rows}, var(--cell-size));"
    );

    view! {
        <div
            class="board"
            style={grid_style}
            on:mouseup=move |_| commit_drag()
            on:mouseleave=move |_| cancel_drag()
        >
            {cells}
        </div>

        <div class="status">
            {move || message.get().map(|m| view! { <span class="error">{m}</span> })}
        </div>

        <button
            class="reset"
            on:click=move |_| {
                placed.set(vec![]);
                message.set(None);
            }
        >
            "Reset"
        </button>
    }
}

fn error_text(e: PlacementError) -> String {
    match e {
        PlacementError::OutOfBounds => "Rechteck verlässt das Spielfeld.".into(),
        PlacementError::NoClue => "Rechteck enthält keine Zahl.".into(),
        PlacementError::MultipleClues => "Rechteck enthält mehr als eine Zahl.".into(),
        PlacementError::AreaMismatch { expected, actual } => {
            format!("Falsche Größe: erwartet {expected} Felder, sind aber {actual}.")
        }
    }
}
