use crate::Coord;

mod physical;

pub use physical::WidgetPhysicalGeometry;

/// A description of the position and size of a widget and how to generate the
/// layout
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WidgetGeometry<T: Coord> {
    /// The position and size of the widget
    physical: WidgetPhysicalGeometry<T>,
}
