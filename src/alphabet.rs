
use num::Integer;
use num_traits::{FromPrimitive, One, ToPrimitive, Zero};
use std::ops::Add;

pub trait Alphabet:
    Integer + Clone + Add<Self, Output = Self> + One + Zero + FromPrimitive + ToPrimitive + Send + Copy
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
            + Copy,
    > Alphabet for T
{}
