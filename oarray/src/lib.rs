extern crate rand;
extern crate streaming_iterator;

pub mod oarray;
mod fitness;
pub mod t_combinations;
pub use oarray::OArray;
pub use fitness::FitnessFunction;
mod parse;
pub mod incremental;
pub mod wtform;
pub mod binary_strings;