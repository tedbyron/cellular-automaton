use wasm_bindgen::prelude::*;

/// A list containing values that each represent the number of live,
/// first-generation neighbors that must exist for a dead cell to come to life.
pub type BirthRule = Vec<u8>;

/// A list containing values that each represent the number of live,
/// first-generation neighbors that must exist for a live, first-generation cell
/// to remain alive.
pub type SurvivalRule = Vec<u8>;

/// The maximum value that a cell can have.
pub type GenerationRule = u8;

/// A ruleset consisting of survival, birth, and generation rules.
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Ruleset {
    #[wasm_bindgen(getter_with_clone)]
    pub birth: BirthRule,
    #[wasm_bindgen(getter_with_clone)]
    pub survival: SurvivalRule,
    pub generation: GenerationRule,
}

#[wasm_bindgen]
impl Ruleset {
    /// Constructs a new ruleset for an automaton.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    #[wasm_bindgen(constructor)]
    #[must_use]
    pub fn new(b: &[u8], s: &[u8], c: u8) -> Self {
        Self {
            birth: b.to_vec(),
            survival: s.to_vec(),
            generation: c - 1,
        }
    }

    // /// Constructs a new ruleset from a `&JsString`.
    // ///
    // /// # Errors
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// todo!()
    // /// ```
    // // #[wasm_bindgen]
    // // #[must_use]
    // pub fn from(string: &str) -> Self {
    //     let matches = string.matches(!char::is_ascii_digit);

    //     Self {
    //         birth: matches[0].as_bytes().to_vec(),
    //         survival: matches[1].as_bytes().to_vec(),
    //         generation: matches[2].parse::<u8>().unwrap() - 1,
    //     }
    // }

    // pub fn parse(s: &str) {}
}

impl Default for Ruleset {
    /// Returns a ruleset using rules from Conway's Game of Life.
    fn default() -> Self {
        Self {
            birth: vec![3],
            survival: vec![2, 3],
            generation: 1,
        }
    }
}
