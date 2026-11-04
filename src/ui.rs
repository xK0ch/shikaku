use leptos::prelude::*;

use crate::game::{Coord, PlacementError, Puzzle, Rect};
use crate::i18n::Lang;

const PALETTE: &[&str] = &[
    "#d4a574", "#b8c9a8", "#dcc9b8", "#a8b8c4",
    "#c8b8c4", "#d4b890", "#b8a888", "#9eb8b0",
    "#caa498", "#b0a4b8", "#a8c0a4", "#dabb84",
];

#[component]
pub fn Board(
    puzzle: RwSignal<Puzzle>,
    placed: RwSignal<Vec<Rect>>,
    drag_start: RwSignal<Option<Coord>>,
    drag_end: RwSignal<Option<Coord>>,
    message: RwSignal<Option<String>>,
) -> impl IntoView {
    let lang = use_context::<RwSignal<Lang>>().expect("Lang context must be provided");

    let pending_rect = move || match (drag_start.get(), drag_end.get()) {
        (Some(s), Some(e)) => Some(Rect::new(s.0, s.1, e.0, e.1)),
        _ => None,
    };

    let commit_drag = move || {
        let Some(rect) = pending_rect() else {
            return;
        };
        let texts = lang.get_untracked().texts();
        let overlaps_existing = placed.with(|ps| ps.iter().any(|p| p.overlaps(&rect)));
        if overlaps_existing {
            message.set(Some(texts.err_overlap.to_string()));
        } else {
            let result = puzzle.with(|p| p.validate_placement(&rect));
            match result {
                Ok(_) => {
                    placed.update(|ps| ps.push(rect));
                    message.set(None);
                }
                Err(e) => message.set(Some(error_text(e, texts).to_string())),
            }
        }
        drag_start.set(None);
        drag_end.set(None);
    };

    let cancel_drag = move || {
        drag_start.set(None);
        drag_end.set(None);
    };

    let board_ref = NodeRef::<leptos::html::Div>::new();

    let cell_at = move |client_x: f64, client_y: f64| -> Option<Coord> {
        let board = board_ref.get_untracked()?;
        let (rows, cols) = puzzle.with_untracked(|p| (p.rows, p.cols));
        let inner_width = f64::from(board.client_width());
        let inner_height = f64::from(board.client_height());
        if rows == 0 || cols == 0 || inner_width <= 0.0 || inner_height <= 0.0 {
            return None;
        }
        let bounds = board.get_bounding_client_rect();
        let x = client_x - bounds.left() - f64::from(board.client_left());
        let y = client_y - bounds.top() - f64::from(board.client_top());
        let index_within = |offset: f64, extent: f64, count: usize| {
            let index = (offset / extent * count as f64).floor();
            index.clamp(0.0, (count - 1) as f64) as usize
        };
        Some((index_within(y, inner_height, rows), index_within(x, inner_width, cols)))
    };

    let start_drag_or_remove = move |coord: Coord| {
        let hit = placed.with(|ps| ps.iter().position(|r| r.contains(coord)));
        if let Some(idx) = hit {
            placed.update(|ps| {
                ps.remove(idx);
            });
            message.set(None);
            return false;
        }
        drag_start.set(Some(coord));
        drag_end.set(Some(coord));
        message.set(None);
        true
    };

    let extend_drag = move |coord: Coord| {
        if drag_start.get_untracked().is_some() && drag_end.get_untracked() != Some(coord) {
            drag_end.set(Some(coord));
        }
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
                "--cell-size: min({cell}px, calc((100vw - 5rem) / {c})); \
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
            node_ref=board_ref
            on:pointerdown=move |ev: leptos::ev::PointerEvent| {
                if !ev.is_primary() {
                    return;
                }
                let Some(coord) = cell_at(f64::from(ev.client_x()), f64::from(ev.client_y())) else {
                    return;
                };
                ev.prevent_default();
                if start_drag_or_remove(coord) {
                    if let Some(board) = board_ref.get_untracked() {
                        let _ = board.set_pointer_capture(ev.pointer_id());
                    }
                }
            }
            on:pointermove=move |ev: leptos::ev::PointerEvent| {
                if let Some(coord) = cell_at(f64::from(ev.client_x()), f64::from(ev.client_y())) {
                    extend_drag(coord);
                }
            }
            on:pointerup=move |_| commit_drag()
            on:pointercancel=move |_| cancel_drag()
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
                                        let cr = (p.row_start + p.row_end) / 2;
                                        let cc = (p.col_start + p.col_end) / 2;
                                        if coord == (cr, cc) {
                                            return "background:rgba(200,65,42,0.32);color:#9c2f1c;".to_string();
                                        }
                                        return "background:rgba(200,65,42,0.16);".to_string();
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
                                <div class="cell" style=style>
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

fn error_text(e: PlacementError, texts: crate::i18n::Texts) -> &'static str {
    match e {
        PlacementError::OutOfBounds => texts.err_out_of_bounds,
        PlacementError::NoClue => texts.err_no_clue,
        PlacementError::MultipleClues => texts.err_multiple_clues,
        PlacementError::AreaMismatch { .. } => texts.err_wrong_size,
    }
}
