use std::fmt::Debug;

use crate::{Coord, WidgetBox};

/// All information the genetator can use to construct the new geometry
#[derive(Clone, Debug, PartialEq)]
pub struct WidgetGeometryInfo<'a, T: Coord> {
    /// The absolute coordinates of the viewport this widget is inside
    pub viewport: &'a WidgetBox<T>,
    /// The relative physical geometry of the previous sibling or None if this
    /// is the first widget in this viewport
    pub sibling: Option<&'a WidgetBox<T>>,
}

impl<'a, T: Coord> WidgetGeometryInfo<'a, T> {
    /// Constructs a new geometry info with a sibling
    ///
    /// # Parameters
    ///
    /// viewport: The absolute coordinates of the viewport this widget is inside
    ///
    /// sibling: The relative coordinates of the geometry of the previous
    /// sibling widget
    pub fn with_sibling(viewport: &'a WidgetBox<T>, sibling: &'a WidgetBox<T>) -> Self {
        return Self {
            viewport,
            sibling: Some(sibling),
        };
    }

    /// Constructs a new geometry info without a sibling
    ///
    /// # Parameters
    ///
    /// viewport: The absolute coordinates of the viewport this widget is inside
    pub fn without_sibling(viewport: &'a WidgetBox<T>) -> Self {
        return Self {
            viewport,
            sibling: None,
        };
    }
}

/// A trait for an object to be able to generate the physical geometry for a
/// widget
pub trait WidgetGeometryGenerator<T: Coord>: Debug {
    /// Generates the new relative physical geometry of the widget
    ///
    /// # Parameters
    ///
    /// info: All the info related to other widgets and viewports used to
    /// construct the geometry
    fn generate(&self, info: &WidgetGeometryInfo<T>) -> WidgetBox<T>;
}
