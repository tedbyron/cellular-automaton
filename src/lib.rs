#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

//! A cellular automaton simulation library with `WebAssembly` as a target.

mod automaton_life_like;

pub use automaton_life_like::{ruleset::*, AutomatonLifeLike};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
