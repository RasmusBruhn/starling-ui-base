//!
//! x-coordinate increases to the right while y-coordinate increases downwards.
//!
//! All viewports have an internal local coordinate system used for all children
//! with (0,0) at the upper left corner and (1,1) at the lower right corner
//!

mod primitives;
mod widget_geometry;

pub use primitives::{Coord, Point, WidgetBox};
pub use widget_geometry::{
    WidgetGeometry, WidgetGeometryGenerator, WidgetGeometryInfo, WidgetGeometryUpdateStatus,
    WidgetPhysicalGeometry, geometry,
};

/// A generic widget, the base of all elements in the ui
#[derive(Debug)]
pub struct Widget<T: Coord> {
    /// The description of the geometry of the widget
    geometry: WidgetGeometry<T>,
}
