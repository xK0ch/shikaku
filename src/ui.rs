use leptos::prelude::*;

use crate::game::Puzzle;

#[component]
pub fn Board(puzzle: Puzzle) -> impl IntoView {
    let rows = puzzle.rows;
    let cols = puzzle.cols;
    let clues = puzzle.clues.clone();

    let cells: Vec<_> = (0..rows)
        .flat_map(|r| {
            let clues = clues.clone();
            (0..cols).map(move |c| {
                let value = clues.iter().find(|cl| cl.at == (r, c)).map(|cl| cl.value);
                view! {
                    <div class="cell">
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
        <div class="board" style={grid_style}>
            {cells}
        </div>
    }
}
