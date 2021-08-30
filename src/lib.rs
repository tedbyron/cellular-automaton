#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

//! A cellular automaton simulation library with support for `WebAssembly`.

mod automaton_life_like;
mod rules;

pub use automaton_life_like::Automaton;
pub use rules::{
    bs::RulesetBS,
    bsc::RulesetBSC,
    types::{BirthRule, GenerationRule, SurvivalRule},
};

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
