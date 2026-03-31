use num_traits::Float;
use std::fmt::Debug;

/// The generic coordinate type allowed for all widgets
pub trait Coord: Float + Debug + 'static {}

impl Coord for f32 {}
impl Coord for f64 {}
