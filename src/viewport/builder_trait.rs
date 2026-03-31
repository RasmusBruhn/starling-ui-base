use crate::{Coord, GeometryInfo, Rect, Widget};
use std::fmt::Debug;

pub trait ViewportBuilder<T: Coord>: Debug {
    /// Builds all the widgets for a viewport
    ///
    /// # Parameters
    ///
    /// info: The info used to build the widgets
    ///
    /// viewport: The absolute coordinates of the viewport to put the widgets inside
    fn build(&self, info: &GeometryInfo<T>, viewport: &Rect<T>) -> Vec<Widget<T>>;
}
