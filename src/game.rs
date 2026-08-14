pub type Coord = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Clue {
    pub at: Coord,
    pub value: usize,
}

#[derive(Debug, Clone)]
pub struct Puzzle {
    pub rows: usize,
    pub cols: usize,
    pub clues: Vec<Clue>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub row_start: usize,
    pub col_start: usize,
    pub row_end: usize,
    pub col_end: usize,
}

impl Rect {
    pub fn new(row_a: usize, col_a: usize, row_b: usize, col_b: usize) -> Self {
        let (row_start, row_end) = if row_a <= row_b { (row_a, row_b) } else { (row_b, row_a) };
        let (col_start, col_end) = if col_a <= col_b { (col_a, col_b) } else { (col_b, col_a) };
        Self { row_start, col_start, row_end, col_end }
    }

    pub fn area(&self) -> usize {
        (self.row_end - self.row_start + 1) * (self.col_end - self.col_start + 1)
    }

    pub fn contains(&self, (r, c): Coord) -> bool {
        r >= self.row_start && r <= self.row_end && c >= self.col_start && c <= self.col_end
    }

    pub fn overlaps(&self, other: &Rect) -> bool {
        !(self.row_end < other.row_start
            || other.row_end < self.row_start
            || self.col_end < other.col_start
            || other.col_end < self.col_start)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlacementError {
    OutOfBounds,
    NoClue,
    MultipleClues,
    AreaMismatch { expected: usize, actual: usize },
}

impl Puzzle {
    pub fn fits(&self, rect: &Rect) -> bool {
        rect.row_end < self.rows && rect.col_end < self.cols
    }

    pub fn validate_placement(&self, rect: &Rect) -> Result<Clue, PlacementError> {
        if !self.fits(rect) {
            return Err(PlacementError::OutOfBounds);
        }
        let contained: Vec<Clue> = self.clues.iter().copied().filter(|c| rect.contains(c.at)).collect();
        match contained.len() {
            0 => Err(PlacementError::NoClue),
            1 => {
                let clue = contained[0];
                if rect.area() == clue.value {
                    Ok(clue)
                } else {
                    Err(PlacementError::AreaMismatch { expected: clue.value, actual: rect.area() })
                }
            }
            _ => Err(PlacementError::MultipleClues),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn rect_normalizes_swapped_corners() {
        let r = Rect::new(3, 2, 1, 0);
        assert_eq!(r.row_start, 1);
        assert_eq!(r.row_end, 3);
        assert_eq!(r.col_start, 0);
        assert_eq!(r.col_end, 2);
    }

    #[test]
    fn rect_area_single_cell() {
        assert_eq!(Rect::new(2, 3, 2, 3).area(), 1);
    }

    #[test]
    fn rect_area_3x2() {
        assert_eq!(Rect::new(0, 0, 2, 1).area(), 6);
    }

    #[test]
    fn rect_contains_corners_and_outside() {
        let r = Rect::new(1, 1, 3, 3);
        assert!(r.contains((1, 1)));
        assert!(r.contains((3, 3)));
        assert!(r.contains((2, 2)));
        assert!(!r.contains((0, 2)));
        assert!(!r.contains((4, 2)));
    }

    #[test]
    fn rect_overlap_detection() {
        let a = Rect::new(0, 0, 2, 2);
        assert!(a.overlaps(&Rect::new(2, 2, 4, 4)));
        assert!(!a.overlaps(&Rect::new(3, 3, 4, 4)));
        assert!(!a.overlaps(&Rect::new(0, 3, 2, 5)));
    }

    #[test]
    fn placement_valid_when_area_matches_single_clue() {
        let p = sample_puzzle();
        let r = Rect::new(0, 0, 1, 0);
        assert_eq!(p.validate_placement(&r), Ok(Clue { at: (0, 0), value: 2 }));
    }

    #[test]
    fn placement_fails_when_area_mismatches() {
        let p = sample_puzzle();
        let r = Rect::new(0, 0, 0, 0);
        assert_eq!(
            p.validate_placement(&r),
            Err(PlacementError::AreaMismatch { expected: 2, actual: 1 })
        );
    }

    #[test]
    fn placement_fails_with_no_clue() {
        let p = sample_puzzle();
        let r = Rect::new(2, 2, 2, 2);
        assert_eq!(p.validate_placement(&r), Err(PlacementError::NoClue));
    }

    #[test]
    fn placement_fails_with_multiple_clues() {
        let p = sample_puzzle();
        let r = Rect::new(0, 0, 0, 1);
        assert_eq!(p.validate_placement(&r), Err(PlacementError::MultipleClues));
    }

    #[test]
    fn placement_fails_when_out_of_bounds() {
        let p = sample_puzzle();
        let r = Rect::new(0, 0, 4, 0);
        assert_eq!(p.validate_placement(&r), Err(PlacementError::OutOfBounds));
    }
}
