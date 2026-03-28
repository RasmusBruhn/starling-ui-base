//!
//! x-coordinate increases to the right while y-coordinate increases downwards.
//!
//! All viewports have an internal local coordinate system used for all children
//! with (0,0) at the upper left corner and (1,1) at the lower right corner
//!

use num_traits::Float;

mod geometry;
mod primitives;

pub use geometry::{
    WidgetGeometry, WidgetGeometryGenerator, WidgetGeometryInfo, WidgetGeometryUpdateStatus,
    WidgetPhysicalGeometry,
};
pub use primitives::{Point, WidgetBox};

/// A generic widget, the base of all elements in the ui
#[derive(Debug)]
pub struct Widget<T: Float> {
    /// The description of the geometry of the widget
    geometry: WidgetGeometry<T>,
}
