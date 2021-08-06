use wasm_bindgen::prelude::wasm_bindgen;

/// A list containing values that each represent the number of live,
/// first-generation neighbors that must exist for a live, first-generation cell
/// to remain alive.
pub type SurvivalRule = Vec<u8>;

/// A list containing values that each represent the number of live,
/// first-generation neighbors that must exist for a dead cell to come to life.
pub type BirthRule = Vec<u8>;

/// The maximum value that a cell can have.
pub type GenerationRule = u8;

/// A ruleset consisting of survival, birth, and generation rules.
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct RuleSet {
    #[wasm_bindgen(getter_with_clone)]
    pub survival: SurvivalRule,
    #[wasm_bindgen(getter_with_clone)]
    pub birth: BirthRule,
    pub generation: GenerationRule,
}

#[wasm_bindgen]
impl RuleSet {
    /// Constructs a new ruleset for an automaton.
    #[wasm_bindgen(constructor)]
    #[must_use]
    pub fn new(s: &[u8], b: &[u8], c: u8) -> Self {
        Self {
            survival: s.to_vec(),
            birth: b.to_vec(),
            generation: c - 1,
        }
    }
}

impl Default for RuleSet {
    /// Returns a ruleset using rules from Conway's Game of Life.
    fn default() -> Self {
        Self {
            survival: vec![2, 3],
            birth: vec![3],
            generation: 1,
        }
    }
}
