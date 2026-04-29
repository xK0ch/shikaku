#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    De,
    En,
}

impl Lang {
    pub fn code(self) -> &'static str {
        match self {
            Lang::De => "de",
            Lang::En => "en",
        }
    }

    pub fn texts(self) -> Texts {
        match self {
            Lang::De => Texts {
                tagline_prefix: "Japanisches Logikrätsel",
                size_label: "Größe",
                new_puzzle: "Neues Puzzle",
                reset: "Reset",
                solved: "Gelöst",
                rules_heading: "Regeln",
                rule_1: "Jedes Rechteck enthält genau eine Zahl.",
                rule_2: "Die Fläche des Rechtecks entspricht dieser Zahl.",
                rule_3: "Rechtecke überlappen nicht und decken das ganze Brett ab.",
                err_overlap: "Überlappt mit einem bereits platzierten Rechteck.",
                err_out_of_bounds: "Rechteck verlässt das Spielfeld.",
                err_no_clue: "Rechteck enthält keine Zahl.",
                err_multiple_clues: "Rechteck enthält mehr als eine Zahl.",
                err_wrong_size: "Falsche Größe.",
            },
            Lang::En => Texts {
                tagline_prefix: "Japanese logic puzzle",
                size_label: "Size",
                new_puzzle: "New puzzle",
                reset: "Reset",
                solved: "Solved",
                rules_heading: "Rules",
                rule_1: "Each rectangle contains exactly one number.",
                rule_2: "The area of the rectangle equals that number.",
                rule_3: "Rectangles do not overlap and cover the entire board.",
                err_overlap: "Overlaps an existing rectangle.",
                err_out_of_bounds: "Rectangle leaves the board.",
                err_no_clue: "Rectangle contains no number.",
                err_multiple_clues: "Rectangle contains more than one number.",
                err_wrong_size: "Wrong size.",
            },
        }
    }

    pub fn cannot_generate(self, size: usize) -> String {
        match self {
            Lang::De => format!(
                "Konnte kein {size}\u{00d7}{size}-Puzzle generieren. Bitte erneut versuchen oder eine andere Größe wählen."
            ),
            Lang::En => format!(
                "Could not generate a {size}\u{00d7}{size} puzzle. Please try again or pick a different size."
            ),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Texts {
    pub tagline_prefix: &'static str,
    pub size_label: &'static str,
    pub new_puzzle: &'static str,
    pub reset: &'static str,
    pub solved: &'static str,
    pub rules_heading: &'static str,
    pub rule_1: &'static str,
    pub rule_2: &'static str,
    pub rule_3: &'static str,
    pub err_overlap: &'static str,
    pub err_out_of_bounds: &'static str,
    pub err_no_clue: &'static str,
    pub err_multiple_clues: &'static str,
    pub err_wrong_size: &'static str,
}
