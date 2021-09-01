#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

//! A cellular automaton simulation library with support for `WebAssembly`.

mod automaton_life_like;
mod rule;
mod ruleset;

pub use automaton_life_like::*;
pub use rule::*;
pub use ruleset::*;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
