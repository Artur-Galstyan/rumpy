//! A numerical library built from verified learner implementations.
//! Active exercises remain separate until they graduate into src/.
mod array;
pub use array::{Array, ShapeError};
pub mod creation;
pub use creation::{arange, eye, full, ones, zeros};
pub mod operations;
pub use operations::{mean, reshape, sum, transpose};
