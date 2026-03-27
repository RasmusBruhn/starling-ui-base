use crate::{Box, Coord};

/// A description of the position and size of a widget
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WidgetPhysicalGeometry<T: Coord> {
    /// The position and size of the widget relative to its parent viewport
    relative: Box<T>,
    /// The position and size of the widget relative to its root viewport
    absolute: Box<T>,
}
