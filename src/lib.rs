//! Echos in the Dark - A roguelike game built with Bevy
//!
//! This crate provides the core game functionality and can be used as a library
//! for examples and tests.

// Re-export the main modules
pub mod core;
pub mod debug;
pub mod gameplay;
pub mod prelude;
pub mod rendering;
pub mod ui;
pub mod utils;

// Re-export commonly used items
pub use prelude::*;
