#![warn(clippy::imprecise_flops, clippy::suboptimal_flops)]

pub mod engine;
mod search;

pub use search::{is_repetition_draw, Search, SearchParams, TtEntry, allocate_tt};
