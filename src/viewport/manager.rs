use crate::{Coord, GeometryInfo, Rect};
use std::fmt::Debug;

pub trait ViewportManager<T: Coord>: Debug {
    /// Calculates the new corrdinates of the viewport relative to its parent
    /// widget
    ///
    /// # Parameters
    ///
    /// info: The info for the parent widget and system info, the sibling is
    /// guarenteed to be None
    fn update(&self, info: &GeometryInfo<T>) -> Rect<T>;
}
