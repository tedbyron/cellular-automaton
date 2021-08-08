#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

//! A cellular automaton simulation library targeting `WebAssembly`.

pub mod automaton_life_like;
pub mod rules;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
