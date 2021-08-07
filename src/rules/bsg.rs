//! A ruleset containing birth, survival, and generation rules (B/S/C notation).

use super::{BirthRule, GenerationRule, SurvivalRule};

#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::wasm_bindgen;

cfg_if::cfg_if! {
    if #[cfg(feature = "wasm-bindgen")] {
        /// A ruleset containing birth, survival, and generation rules.
        #[wasm_bindgen]
        #[derive(Clone, Debug, PartialEq, PartialOrd)]
        pub struct Ruleset {
            #[wasm_bindgen(getter_with_clone)]
            pub birth: BirthRule,
            #[wasm_bindgen(getter_with_clone)]
            pub survival: SurvivalRule,
            #[wasm_bindgen(getter)]
            pub generation: GenerationRule,
        }
    } else {
        /// A ruleset containing birth, survival, and generation rules.
        #[derive(Clone, Debug, PartialEq, PartialOrd)]
        pub struct Ruleset {
            pub birth: BirthRule,
            pub survival: SurvivalRule,
            pub generation: GenerationRule,
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
    pub fn new(b: &[i8], s: &[i8], c: i8) -> Self {
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        Self {
            birth: b.to_vec(),
            survival: s.to_vec(),
            generation: c - 1,
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

    /// Sets the generation rule.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    #[cfg_attr(feature = "wasm-bindgen", wasm_bindgen(setter, js_name = setGenerationRule))]
    pub fn set_generation_rule(&mut self, c: i8) {
        if c >= 2 {
            self.generation = c - 1;
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
    //         generation: matches[2].parse::<i8>().unwrap() - 1,
    //     }
    // }

    // pub fn parse(s: &str) {}
}

impl Default for Ruleset {
    /// Rules from Conway's Game of Life.
    fn default() -> Self {
        Self {
            birth: vec![3],
            survival: vec![2, 3],
            generation: 1,
        }
    }
}
