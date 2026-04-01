use crate::{Coord, GeometryInfo, Rect};
use std::fmt::Debug;

/// A trait for an object to be able to generate the physical geometry for a
/// widget
pub trait GeometryGeneratorTrait<T: Coord>: Debug {
    /// Generates the new relative physical geometry of the widget
    ///
    /// # Parameters
    ///
    /// info: All the info related to other widgets and viewports used to
    /// construct the geometry
    fn generate(&self, info: &GeometryInfo<T>) -> Rect<T>;
}

pub type GeometryGenerator<T> = Box<dyn GeometryGeneratorTrait<T>>;
