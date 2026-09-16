//! A numerical library built one exercise at a time.
//! Exercise implementations are re-exported here, not copied into a second library.
extern crate self as rumpy;
mod array;
pub use array::{Array, ShapeError};

#[path = "../exercises/01_creation/zeros.rs"]
mod zeros_exercise;
pub use zeros_exercise::zeros;
