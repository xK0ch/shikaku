use crate::game::{Clue, Puzzle, Rect};

#[derive(Debug, PartialEq, Eq)]
pub enum SolveResult {
    None,
    Unique(Vec<Rect>),
    Multiple,
}

pub fn solve(puzzle: &Puzzle) -> SolveResult {
    let total_cells = puzzle.rows * puzzle.cols;
    let clue_sum: usize = puzzle.clues.iter().map(|c| c.value).sum();
    if clue_sum != total_cells {
        return SolveResult::None;
    }

    let mut clue_options: Vec<(Clue, Vec<Rect>)> = puzzle
        .clues
        .iter()
        .map(|c| (*c, candidates_for_clue(puzzle, c)))
        .collect();

    if clue_options.iter().any(|(_, opts)| opts.is_empty()) {
        return SolveResult::None;
    }

    clue_options.sort_by_key(|(_, opts)| opts.len());

    let mut placed: Vec<Rect> = Vec::with_capacity(clue_options.len());
    let mut covered = vec![false; total_cells];
    let mut first_solution: Option<Vec<Rect>> = None;
    let mut solutions_found: usize = 0;

    backtrack(
        0,
        &clue_options,
        puzzle.cols,
        &mut placed,
        &mut covered,
        &mut first_solution,
        &mut solutions_found,
    );

    match (solutions_found, first_solution) {
        (0, _) => SolveResult::None,
        (1, Some(s)) => SolveResult::Unique(s),
        _ => SolveResult::Multiple,
    }
}

fn backtrack(
    idx: usize,
    clue_options: &[(Clue, Vec<Rect>)],
    cols: usize,
    placed: &mut Vec<Rect>,
    covered: &mut [bool],
    first_solution: &mut Option<Vec<Rect>>,
    solutions_found: &mut usize,
) -> bool {
    if *solutions_found >= 2 {
        return true;
    }
    if idx == clue_options.len() {
        if first_solution.is_none() {
            *first_solution = Some(placed.clone());
        }
        *solutions_found += 1;
        return *solutions_found >= 2;
    }

    let (_, options) = &clue_options[idx];
    for rect in options {
        if rect_overlaps_covered(rect, cols, covered) {
            continue;
        }
        mark(rect, cols, covered, true);
        placed.push(*rect);
        let stop = backtrack(idx + 1, clue_options, cols, placed, covered, first_solution, solutions_found);
        placed.pop();
        mark(rect, cols, covered, false);
        if stop {
            return true;
        }
    }
    false
}

fn rect_overlaps_covered(rect: &Rect, cols: usize, covered: &[bool]) -> bool {
    for r in rect.row_start..=rect.row_end {
        for c in rect.col_start..=rect.col_end {
            if covered[r * cols + c] {
                return true;
            }
        }
    }
    false
}

fn mark(rect: &Rect, cols: usize, covered: &mut [bool], value: bool) {
    for r in rect.row_start..=rect.row_end {
        for c in rect.col_start..=rect.col_end {
            covered[r * cols + c] = value;
        }
    }
}

pub(crate) fn candidates_for_clue(puzzle: &Puzzle, clue: &Clue) -> Vec<Rect> {
    let mut result = Vec::new();
    let value = clue.value;
    let (cr, cc) = clue.at;

    for h in 1..=value {
        if value % h != 0 {
            continue;
        }
        let w = value / h;
        if h > puzzle.rows || w > puzzle.cols {
            continue;
        }

        let row_start_min = cr.saturating_sub(h - 1);
        let row_start_max = cr.min(puzzle.rows - h);
        let col_start_min = cc.saturating_sub(w - 1);
        let col_start_max = cc.min(puzzle.cols - w);

        for rs in row_start_min..=row_start_max {
            for cs in col_start_min..=col_start_max {
                let rect = Rect::new(rs, cs, rs + h - 1, cs + w - 1);
                let hits_other_clue = puzzle
                    .clues
                    .iter()
                    .any(|c2| c2.at != clue.at && rect.contains(c2.at));
                if !hits_other_clue {
                    result.push(rect);
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Clue;

    fn sample_4x4() -> Puzzle {
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

    #[test]
    fn unique_solution_for_known_4x4() {
        let p = sample_4x4();
        let result = solve(&p);
        assert!(matches!(result, SolveResult::Unique(_)), "expected unique, got {result:?}");
    }

    #[test]
    fn solution_covers_every_cell_exactly_once() {
        let p = sample_4x4();
        let SolveResult::Unique(rects) = solve(&p) else {
            panic!("expected unique");
        };
        let mut covered = vec![0u32; p.rows * p.cols];
        for rect in &rects {
            for r in rect.row_start..=rect.row_end {
                for c in rect.col_start..=rect.col_end {
                    covered[r * p.cols + c] += 1;
                }
            }
        }
        assert!(covered.iter().all(|&n| n == 1));
    }

    #[test]
    fn no_solution_when_clue_sum_mismatches_grid() {
        let p = Puzzle {
            rows: 4,
            cols: 4,
            clues: vec![Clue { at: (0, 0), value: 5 }],
        };
        assert_eq!(solve(&p), SolveResult::None);
    }

    #[test]
    fn multiple_solutions_when_under_constrained() {
        let p = Puzzle {
            rows: 2,
            cols: 2,
            clues: vec![
                Clue { at: (0, 0), value: 2 },
                Clue { at: (1, 1), value: 2 },
            ],
        };
        assert_eq!(solve(&p), SolveResult::Multiple);
    }

    #[test]
    fn candidates_respect_grid_and_other_clues() {
        let p = sample_4x4();
        let candidates = candidates_for_clue(&p, &Clue { at: (0, 0), value: 2 });
        assert!(candidates.iter().all(|r| r.area() == 2));
        assert!(candidates.iter().all(|r| r.contains((0, 0))));
        assert!(candidates.iter().all(|r| !r.contains((0, 1))));
    }
}
