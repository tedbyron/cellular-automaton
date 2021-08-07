//! A ruleset containing birth and survival rules (B/S notation).

use super::{BirthRule, SurvivalRule};

#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::wasm_bindgen;

cfg_if::cfg_if! {
    if #[cfg(feature = "wasm-bindgen")] {
        /// A ruleset containing birth and survival rules.
        #[wasm_bindgen]
        #[derive(Clone, Debug, PartialEq, PartialOrd)]
        pub struct Ruleset {
            #[wasm_bindgen(getter_with_clone)]
            pub birth: BirthRule,
            #[wasm_bindgen(getter_with_clone)]
            pub survival: SurvivalRule,
        }
    } else {
        /// A ruleset containing birth and survival rules.
        #[derive(Clone, Debug, PartialEq, PartialOrd)]
        pub struct Ruleset {
            pub birth: BirthRule,
            pub survival: SurvivalRule,
        }
    }
}

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
impl Ruleset {
    /// Constructs a new ruleset containing birth, survival, and generation rules.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    #[cfg_attr(feature = "wasm-bindgen", wasm_bindgen(constructor))]
    #[must_use]
    pub fn new(b: &[i8], s: &[i8]) -> Self {
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        Self {
            birth: b.to_vec(),
            survival: s.to_vec(),
        }
    }

    /// Sets the birth rule.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    #[cfg_attr(feature = "wasm-bindgen", wasm_bindgen(setter, js_name = setBirthRule))]
    pub fn set_birth_rule(&mut self, b: &[i8]) {
        if b.iter().all(|n| (1..=8).contains(n)) {
            self.birth = b.to_vec();
        }
    }

    /// Sets the survival rule.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    #[cfg_attr(feature = "wasm-bindgen", wasm_bindgen(setter, js_name = setSurvivalRule))]
    pub fn set_survival_rule(&mut self, s: &[i8]) {
        if s.iter().all(|n| (-1..=8).contains(n)) {
            self.survival = s.to_vec();
        }
    }
}

impl Default for Ruleset {
    /// Rules from Conway's Game of Life.
    fn default() -> Self {
        Self {
            birth: vec![3],
            survival: vec![2, 3],
        }
    }
}
