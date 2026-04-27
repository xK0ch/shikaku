use rand::seq::SliceRandom;
use rand::Rng;

use crate::game::{Clue, Coord, Puzzle, Rect};
use crate::solver::{solve, SolveResult};

const MIN_RECT_AREA: usize = 2;

pub fn generate(rows: usize, cols: usize, attempts: usize) -> Option<Puzzle> {
    let mut rng = rand::thread_rng();
    for _ in 0..attempts {
        let rects = random_tiling(rows, cols, &mut rng);
        let clues = clues_from_tiling(&rects, &mut rng);
        let puzzle = Puzzle { rows, cols, clues };
        if matches!(solve(&puzzle), SolveResult::Unique(_)) {
            return Some(puzzle);
        }
    }
    None
}

fn random_tiling<R: Rng>(rows: usize, cols: usize, rng: &mut R) -> Vec<Rect> {
    let mut covered = vec![false; rows * cols];
    let mut rects = Vec::new();

    while let Some(idx) = covered.iter().position(|&c| !c) {
        let cr = idx / cols;
        let cc = idx % cols;

        let max_w = (cc..cols)
            .take_while(|&c| !covered[cr * cols + c])
            .count();

        let mut max_h_per_w = vec![0usize; max_w + 1];
        for w in 1..=max_w {
            let mut h = 0;
            'rows: while cr + h < rows {
                for c in cc..cc + w {
                    if covered[(cr + h) * cols + c] {
                        break 'rows;
                    }
                }
                h += 1;
            }
            max_h_per_w[w] = h;
        }

        let (w, h) = pick_size(max_w, &max_h_per_w, rng);

        let rect = Rect::new(cr, cc, cr + h - 1, cc + w - 1);
        for r in rect.row_start..=rect.row_end {
            for c in rect.col_start..=rect.col_end {
                covered[r * cols + c] = true;
            }
        }
        rects.push(rect);
    }
    rects
}

fn pick_size<R: Rng>(max_w: usize, max_h_per_w: &[usize], rng: &mut R) -> (usize, usize) {
    let mut feasible: Vec<(usize, usize)> = Vec::new();
    for w in 1..=max_w {
        for h in 1..=max_h_per_w[w] {
            if w * h >= MIN_RECT_AREA {
                feasible.push((w, h));
            }
        }
    }
    if feasible.is_empty() {
        return (1, 1);
    }
    *feasible.choose(rng).unwrap()
}

fn clues_from_tiling<R: Rng>(rects: &[Rect], rng: &mut R) -> Vec<Clue> {
    rects
        .iter()
        .map(|r| {
            let cells: Vec<Coord> = (r.row_start..=r.row_end)
                .flat_map(|row| (r.col_start..=r.col_end).map(move |c| (row, c)))
                .collect();
            let chosen = *cells.choose(rng).unwrap();
            Clue {
                at: chosen,
                value: r.area(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solver::{solve, SolveResult};

    #[test]
    fn generated_puzzle_has_unique_solution() {
        let p = generate(5, 5, 500).expect("generator should succeed for 5x5");
        assert_eq!(p.rows, 5);
        assert_eq!(p.cols, 5);
        let clue_sum: usize = p.clues.iter().map(|c| c.value).sum();
        assert_eq!(clue_sum, 25);
        assert!(matches!(solve(&p), SolveResult::Unique(_)));
    }

    #[test]
    fn generated_clues_are_within_grid() {
        let p = generate(6, 6, 500).expect("generator should succeed for 6x6");
        for clue in &p.clues {
            assert!(clue.at.0 < p.rows);
            assert!(clue.at.1 < p.cols);
            assert!(clue.value >= 1);
        }
    }

    #[test]
    fn random_tiling_covers_grid_exactly() {
        let mut rng = rand::thread_rng();
        let rects = random_tiling(5, 5, &mut rng);
        let mut covered = vec![0u32; 25];
        for r in &rects {
            for row in r.row_start..=r.row_end {
                for col in r.col_start..=r.col_end {
                    covered[row * 5 + col] += 1;
                }
            }
        }
        assert!(covered.iter().all(|&n| n == 1));
    }
}
