#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

//! A cellular automaton simulation library targeting `WebAssembly`.

mod automaton_life_like;
mod rules;

pub use automaton_life_like::Automaton;
pub use rules::{
    bs::RulesetBS,
    bsc::RulesetBSC,
    types::{BirthRule, GenerationRule, SurvivalRule},
};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
