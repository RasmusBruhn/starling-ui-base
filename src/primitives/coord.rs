use num_traits::FromPrimitive;
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// A trait representing numeric types suitable for coordinates.
/// Typically implemented by f32, f64, and similar numeric types.
pub trait Coord:
    Clone
    + Copy
    + Debug
    + PartialEq
    + Neg<Output = Self>
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + FromPrimitive
    + Sized
{
}

// Implement Coord for common numeric types
impl Coord for f32 {}
impl Coord for f64 {}
