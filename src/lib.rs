//!
//! x-coordinate increases to the right while y-coordinate increases downwards.
//!
//! All viewports have an internal local coordinate system used for all children
//! with (0,0) at the upper left corner and (1,1) at the lower right corner
//!

mod geometry;

pub use geometry::{Box, Coord, Point};

/// A description of the position and size of a widget
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WidgetPhysicalGeometry<T: Coord> {
    /// The position and size of the widget relative to its parent viewport
    relative: Box<T>,
    /// The position and size of the widget relative to its root viewport
    absolute: Box<T>,
}

/// A description of the position and size of a widget and how to generate the
/// layout
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WidgetGeometry<T: Coord> {
    /// The position and size of the widget
    physical: WidgetPhysicalGeometry<T>,
}

/// A generic widget, the base of all elements in the ui
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Widget<T: Coord> {
    /// The description of the geometry of the widget
    geometry: WidgetGeometry<T>,
}
