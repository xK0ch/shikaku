use std::collections::BTreeMap;

use rand::seq::{IndexedRandom, SliceRandom};
use rand::{Rng, RngExt};

use crate::game::{Clue, Coord, Puzzle, Rect};
use crate::solver::{candidates_for_clue, solve, SolveResult};

const SMALL_AREA_PREFERENCE: f64 = 0.5;
const ATTEMPTS_BEFORE_RELAXING: usize = 25;
const CONSTRAINING_CLUES_ABOVE_CELLS: usize = 400;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AreaLimits {
    pub min: usize,
    pub max: usize,
}

impl AreaLimits {
    fn allows(&self, area: usize) -> bool {
        (self.min..=self.max).contains(&area)
    }

    fn relaxed(self, board_area: usize) -> Self {
        let raised_max = (self.max + self.max / 2).max(self.max + 1);
        Self {
            min: self.min,
            max: raised_max.min(board_area).max(self.max),
        }
    }
}

pub fn generate(rows: usize, cols: usize, limits: AreaLimits, attempts: usize) -> Option<Puzzle> {
    let mut rng = rand::rng();
    let mut current_limits = limits;
    for attempt in 1..=attempts {
        let rects = random_tiling(rows, cols, current_limits, &mut rng);
        let mut puzzle = Puzzle { rows, cols, clues: clues_from_tiling(&rects, &mut rng) };
        if rows * cols > CONSTRAINING_CLUES_ABOVE_CELLS {
            move_clues_to_most_constraining_corners(&mut puzzle, &rects, &mut rng);
        }
        if matches!(solve(&puzzle), SolveResult::Unique(_)) {
            return Some(puzzle);
        }
        if attempt % ATTEMPTS_BEFORE_RELAXING == 0 {
            current_limits = current_limits.relaxed(rows * cols);
        }
    }
    None
}

fn random_tiling<R: Rng>(rows: usize, cols: usize, limits: AreaLimits, rng: &mut R) -> Vec<Rect> {
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

        let (w, h) = pick_size(max_w, &max_h_per_w, limits, rng);

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

fn fitting_shapes(max_w: usize, max_h_per_w: &[usize]) -> impl Iterator<Item = (usize, usize)> + '_ {
    (1..=max_w).flat_map(move |w| (1..=max_h_per_w[w]).map(move |h| (w, h)))
}

fn pick_size<R: Rng>(
    max_w: usize,
    max_h_per_w: &[usize],
    limits: AreaLimits,
    rng: &mut R,
) -> (usize, usize) {
    let mut shapes_by_area: BTreeMap<usize, Vec<(usize, usize)>> = BTreeMap::new();
    for (w, h) in fitting_shapes(max_w, max_h_per_w) {
        if limits.allows(w * h) {
            shapes_by_area.entry(w * h).or_default().push((w, h));
        }
    }

    let Some(area) = pick_area(&shapes_by_area.keys().copied().collect::<Vec<_>>(), rng) else {
        return fitting_shapes(max_w, max_h_per_w)
            .max_by_key(|&(w, h)| w * h)
            .unwrap_or((1, 1));
    };
    *shapes_by_area[&area].choose(rng).unwrap()
}

fn pick_area<R: Rng>(areas: &[usize], rng: &mut R) -> Option<usize> {
    let weights: Vec<f64> = areas
        .iter()
        .map(|&area| (area as f64).powf(-SMALL_AREA_PREFERENCE))
        .collect();
    let total: f64 = weights.iter().sum();
    let mut remaining = rng.random::<f64>() * total;
    for (&area, &weight) in areas.iter().zip(&weights) {
        if remaining < weight {
            return Some(area);
        }
        remaining -= weight;
    }
    areas.last().copied()
}

fn corners(rect: &Rect) -> [Coord; 4] {
    [
        (rect.row_start, rect.col_start),
        (rect.row_start, rect.col_end),
        (rect.row_end, rect.col_start),
        (rect.row_end, rect.col_end),
    ]
}

fn move_clues_to_most_constraining_corners<R: Rng>(puzzle: &mut Puzzle, rects: &[Rect], rng: &mut R) {
    for (index, rect) in rects.iter().enumerate() {
        let mut options = corners(rect);
        options.shuffle(rng);
        let best = options
            .into_iter()
            .min_by_key(|&corner| {
                puzzle.clues[index].at = corner;
                candidates_for_clue(puzzle, &puzzle.clues[index]).len()
            })
            .unwrap();
        puzzle.clues[index].at = best;
    }
}

fn clues_from_tiling<R: Rng>(rects: &[Rect], rng: &mut R) -> Vec<Clue> {
    rects
        .iter()
        .map(|r| {
            // Place clues in one of the rectangle's four corners. Corner
            // positions are the most constraining placements because they
            // anchor one corner of the rectangle, which makes the resulting
            // puzzles uniquely solvable far more often than placing the
            // clue in the rectangle's interior.
            let chosen = *corners(r).choose(rng).unwrap();
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

    const SMALL_BOARD: AreaLimits = AreaLimits { min: 2, max: 8 };
    const MEDIUM_BOARD: AreaLimits = AreaLimits { min: 3, max: 15 };

    #[test]
    fn generated_puzzle_has_unique_solution() {
        let p = generate(5, 5, SMALL_BOARD, 500).expect("generator should succeed for 5x5");
        assert_eq!(p.rows, 5);
        assert_eq!(p.cols, 5);
        let clue_sum: usize = p.clues.iter().map(|c| c.value).sum();
        assert_eq!(clue_sum, 25);
        assert!(matches!(solve(&p), SolveResult::Unique(_)));
    }

    #[test]
    fn generated_clues_are_within_grid() {
        let p = generate(6, 6, SMALL_BOARD, 500).expect("generator should succeed for 6x6");
        for clue in &p.clues {
            assert!(clue.at.0 < p.rows);
            assert!(clue.at.1 < p.cols);
            assert!(clue.value >= 1);
        }
    }

    #[test]
    fn clues_are_placed_in_rectangle_corners() {
        let mut rng = rand::rng();
        let rects = random_tiling(8, 8, SMALL_BOARD, &mut rng);
        let clues = clues_from_tiling(&rects, &mut rng);
        for (clue, rect) in clues.iter().zip(rects.iter()) {
            let (r, c) = clue.at;
            let on_corner = (r == rect.row_start || r == rect.row_end)
                && (c == rect.col_start || c == rect.col_end);
            assert!(on_corner, "clue {:?} not at corner of {:?}", clue.at, rect);
        }
    }

    #[test]
    fn random_tiling_covers_grid_exactly() {
        let mut rng = rand::rng();
        let rects = random_tiling(5, 5, SMALL_BOARD, &mut rng);
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

    #[test]
    fn random_tiling_never_exceeds_the_maximum_area() {
        let mut rng = rand::rng();
        for _ in 0..200 {
            let rects = random_tiling(10, 10, MEDIUM_BOARD, &mut rng);
            assert!(rects.iter().all(|r| r.area() <= MEDIUM_BOARD.max));
        }
    }

    #[test]
    fn random_tiling_mixes_small_medium_and_large_areas() {
        let mut rng = rand::rng();
        let areas: Vec<usize> = (0..200)
            .flat_map(|_| random_tiling(10, 10, MEDIUM_BOARD, &mut rng))
            .map(|r| r.area())
            .collect();
        let share = |range: std::ops::RangeInclusive<usize>| {
            areas.iter().filter(|&&area| range.contains(&area)).count() as f64 / areas.len() as f64
        };
        let mean = areas.iter().sum::<usize>() as f64 / areas.len() as f64;
        assert!((3.5..=8.0).contains(&mean), "mean area {mean}");
        assert!(share(1..=3) > 0.15, "small share {}", share(1..=3));
        assert!(share(4..=9) > 0.35, "medium share {}", share(4..=9));
        assert!(share(10..=15) > 0.05, "large share {}", share(10..=15));
    }

    #[test]
    fn constraining_corners_keep_clues_in_corners_with_unchanged_values() {
        let mut rng = rand::rng();
        let limits = AreaLimits { min: 7, max: 20 };
        let rects = random_tiling(30, 30, limits, &mut rng);
        let mut puzzle = Puzzle { rows: 30, cols: 30, clues: clues_from_tiling(&rects, &mut rng) };
        move_clues_to_most_constraining_corners(&mut puzzle, &rects, &mut rng);
        assert_eq!(puzzle.clues.len(), rects.len());
        for (clue, rect) in puzzle.clues.iter().zip(&rects) {
            assert!(corners(rect).contains(&clue.at), "clue {:?} not at corner of {:?}", clue.at, rect);
            assert_eq!(clue.value, rect.area());
        }
    }

    #[test]
    fn large_board_generates_a_unique_puzzle_within_the_limits() {
        let limits = AreaLimits { min: 7, max: 20 };
        let p = generate(30, 30, limits, 25).expect("generator should succeed for 30x30");
        assert!(p.clues.iter().all(|c| c.value <= limits.max));
        assert!(matches!(solve(&p), SolveResult::Unique(_)));
    }

    #[test]
    fn relaxed_limits_grow_up_to_the_board_area() {
        let mut limits = MEDIUM_BOARD;
        let mut maxima = vec![limits.max];
        while limits.max < 100 {
            let next = limits.relaxed(100);
            assert!(next.max > limits.max);
            assert_eq!(next.min, limits.min);
            limits = next;
            maxima.push(limits.max);
        }
        assert_eq!(maxima, vec![15, 22, 33, 49, 73, 100]);
        assert_eq!(limits.relaxed(100), limits);
    }

    #[test]
    fn pick_area_only_returns_offered_areas() {
        let mut rng = rand::rng();
        let areas = [3, 4, 6, 9];
        for _ in 0..1000 {
            let area = pick_area(&areas, &mut rng).unwrap();
            assert!(areas.contains(&area));
        }
        assert_eq!(pick_area(&[], &mut rng), None);
    }

    #[test]
    fn pick_size_uses_the_largest_shape_when_space_is_too_small() {
        let mut rng = rand::rng();
        assert_eq!(pick_size(1, &[0, 1], MEDIUM_BOARD, &mut rng), (1, 1));
        assert_eq!(pick_size(2, &[0, 1, 1], MEDIUM_BOARD, &mut rng), (2, 1));
    }
}
