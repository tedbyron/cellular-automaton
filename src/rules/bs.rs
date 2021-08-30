#[cfg(all(feature = "wasm-bindgen", target_arch = "wasm32"))]
use wasm_bindgen::prelude::wasm_bindgen;

use super::types::{BirthRule, GenerationRule, SurvivalRule};

// expected non-macro attribute, found attribute macro `wasm_bindgen` not a
// non-macro attribute
cfg_if::cfg_if! {
    if #[cfg(all(feature = "wasm-bindgen", target_arch = "wasm32"))] {
        /// A ruleset containing birth and survival (B/S) rules.
        #[wasm_bindgen]
        #[derive(Clone, Debug, PartialEq, PartialOrd)]
        pub struct RulesetBS {
            #[wasm_bindgen(getter_with_clone)]
            pub birth: BirthRule,
            #[wasm_bindgen(getter_with_clone)]
            pub survival: SurvivalRule,
            generation: GenerationRule,
        }
    } else {
        /// A ruleset containing birth and survival (B/S) rules.
        #[derive(Clone, Debug, PartialEq, PartialOrd)]
        pub struct RulesetBS {
            pub birth: BirthRule,
            pub survival: SurvivalRule,
            generation: GenerationRule,
        }
    }
}

#[cfg_attr(all(feature = "wasm-bindgen", target_arch = "wasm32"), wasm_bindgen)]
impl RulesetBS {
    /// Constructs a new ruleset containing birth, survival, and generation rules.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    #[cfg_attr(
        all(feature = "wasm-bindgen", target_arch = "wasm32"),
        wasm_bindgen(constructor)
    )]
    #[must_use]
    pub fn new(b: &[i8], s: &[i8]) -> Self {
        #[cfg(all(feature = "console_error_panic_hook", target_arch = "wasm32"))]
        console_error_panic_hook::set_once();

        Self {
            birth: b.to_vec(),
            survival: s.to_vec(),
            generation: 1,
        }
    }

    /// Sets the birth rule.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    #[cfg_attr(all(feature = "wasm-bindgen", target_arch = "wasm32"), wasm_bindgen(setter, js_name = setBirthRule))]
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
    #[cfg_attr(all(feature = "wasm-bindgen", target_arch = "wasm32"), wasm_bindgen(setter, js_name = setSurvivalRule))]
    pub fn set_survival_rule(&mut self, s: &[i8]) {
        if s.iter().all(|n| (-1..=8).contains(n)) {
            self.survival = s.to_vec();
        }
    }
}

impl Default for RulesetBS {
    /// Rules from Conway's Game of Life.
    fn default() -> Self {
        Self {
            birth: vec![3],
            survival: vec![2, 3],
            generation: 1,
        }
    }
}
