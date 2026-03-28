use crate::{WidgetBox, Coord};

/// A description of the position and size of a widget
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WidgetPhysicalGeometry<T: Coord> {
    /// The position and size of the widget relative to its parent viewport
    pub relative: WidgetBox<T>,
    /// The position and size of the widget relative to its root viewport
    pub absolute: WidgetBox<T>,
}

impl<T: Coord> WidgetPhysicalGeometry<T> {
    /// Constructs a new physical widget geometry
    ///
    /// # Parameters
    ///
    /// relative: The position and size of the widget relative to its parent viewport
    ///
    /// absolute: The position and size of the widget relative to its root viewport
    pub fn new(relative: WidgetBox<T>, absolute: WidgetBox<T>) -> Self {
        return Self { relative, absolute };
    }

    /// Constructs a new physical widget geometry using the absolute geometry of
    /// its parent viewport
    ///
    /// # Parameters
    ///
    /// relative: The position and size of the widget relative to its parent viewport
    ///
    /// parent: The position and size of the parent widget relative to its parent viewport
    pub fn from_parent(relative: WidgetBox<T>, parent: &WidgetBox<T>) -> Self {
        let parent_size = parent.get_size();
        let size = relative.get_size() * parent_size;
        let ll = relative.ll * parent_size + parent.ll;
        let ur = ll + size;
        let absolute = WidgetBox::new(ll, ur);

        return Self::new(relative, absolute);
    }
}
