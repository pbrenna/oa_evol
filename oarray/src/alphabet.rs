
use num::Integer;
use num_traits::{FromPrimitive, One, ToPrimitive, Zero};
use std::ops::Add;
use std::fmt::Debug;

pub trait Alphabet:
    Integer + Clone + Add<Self, Output = Self> + One + Zero + FromPrimitive + ToPrimitive + Send + Copy + Debug
{
}
impl<
        T: Integer
            + Clone
            + Add<Self, Output = Self>
            + One
            + Zero
            + FromPrimitive
            + ToPrimitive
            + Send
            + Copy
            + Debug,
    > Alphabet for T
{
}