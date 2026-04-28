use leptos::prelude::*;

use crate::game::{Coord, PlacementError, Puzzle, Rect};

const PALETTE: &[&str] = &[
    "#fde2e2", "#dceefb", "#e2f5d8", "#fff1c2", "#ead4f5", "#d3f0ee", "#ffd9bd", "#e9e4d4",
    "#f5c6c6", "#c9e2ff", "#d4f0c2", "#fce5b3",
];

#[component]
pub fn Board(
    puzzle: RwSignal<Puzzle>,
    placed: RwSignal<Vec<Rect>>,
    drag_start: RwSignal<Option<Coord>>,
    drag_end: RwSignal<Option<Coord>>,
    message: RwSignal<Option<String>>,
) -> impl IntoView {
    let pending_rect = move || match (drag_start.get(), drag_end.get()) {
        (Some(s), Some(e)) => Some(Rect::new(s.0, s.1, e.0, e.1)),
        _ => None,
    };

    let commit_drag = move || {
        let Some(rect) = pending_rect() else {
            return;
        };
        let overlaps_existing = placed.with(|ps| ps.iter().any(|p| p.overlaps(&rect)));
        if overlaps_existing {
            message.set(Some("Überlappt mit einem bereits platzierten Rechteck.".into()));
        } else {
            let result = puzzle.with(|p| p.validate_placement(&rect));
            match result {
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

    let board_style = move || {
        puzzle.with(|p| {
            let cell_size_px = match p.rows.max(p.cols) {
                0..=5 => 56,
                6..=10 => 40,
                11..=20 => 28,
                21..=30 => 20,
                _ => 16,
            };
            format!(
                "--cell-size: {cell}px; \
                 grid-template-columns: repeat({c}, var(--cell-size)); \
                 grid-template-rows: repeat({r}, var(--cell-size));",
                cell = cell_size_px,
                c = p.cols,
                r = p.rows,
            )
        })
    };

    view! {
        <div
            class="board"
            style=board_style
            on:mouseup=move |_| commit_drag()
            on:mouseleave=move |_| cancel_drag()
        >
            {move || {
                let p = puzzle.get();
                let rows = p.rows;
                let cols = p.cols;
                let clues = p.clues;

                (0..rows)
                    .flat_map(|r| {
                        let clues = clues.clone();
                        (0..cols).map(move |c| {
                            let coord = (r, c);
                            let value = clues.iter().find(|cl| cl.at == coord).map(|cl| cl.value);

                            let style = move || {
                                if let Some(idx) = placed
                                    .with(|ps| ps.iter().position(|rect| rect.contains(coord)))
                                {
                                    return format!("background:{};", PALETTE[idx % PALETTE.len()]);
                                }
                                if let Some(p) = pending_rect() {
                                    if p.contains(coord) {
                                        return "background:rgba(110,170,240,0.35);".to_string();
                                    }
                                }
                                String::new()
                            };

                            let content = move || {
                                if let Some(rect) = pending_rect() {
                                    let cr = (rect.row_start + rect.row_end) / 2;
                                    let cc = (rect.col_start + rect.col_end) / 2;
                                    if coord == (cr, cc) {
                                        let h = rect.row_end - rect.row_start + 1;
                                        let w = rect.col_end - rect.col_start + 1;
                                        return format!("{h}:{w}");
                                    }
                                }
                                value.map(|v| v.to_string()).unwrap_or_default()
                            };

                            view! {
                                <div
                                    class="cell"
                                    style=style
                                    on:mousedown=move |_| {
                                        let hit = placed
                                            .with(|ps| ps.iter().position(|r| r.contains(coord)));
                                        if let Some(idx) = hit {
                                            placed.update(|ps| {
                                                ps.remove(idx);
                                            });
                                            message.set(None);
                                            return;
                                        }
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
                                    {content}
                                </div>
                            }
                        })
                    })
                    .collect_view()
            }}
        </div>
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
