use crate::{Box, Coord};

/// A description of the position and size of a widget
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WidgetPhysicalGeometry<T: Coord> {
    /// The position and size of the widget relative to its parent viewport
    relative: Box<T>,
    /// The position and size of the widget relative to its root viewport
    absolute: Box<T>,
}

impl<T: Coord> WidgetPhysicalGeometry<T> {
    /// Constructs a new physical widget geometry
    ///
    /// # Parameters
    ///
    /// relative: The position and size of the widget relative to its parent viewport
    ///
    /// absolute: The position and size of the widget relative to its root viewport
    pub fn new(relative: Box<T>, absolute: Box<T>) -> Self {
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
    pub fn from_parent(relative: Box<T>, parent: &Box<T>) -> Self {
        let parent_size = parent.get_size();
        let size = relative.get_size() * parent_size;
        let ll = relative.ll * parent_size + parent.ll;
        let ur = ll + size;
        let absolute = Box::new(ll, ur);

        return Self::new(relative, absolute);
    }

    /// Retrieves the position and size of the widget relative to its parent viewport
    pub fn get_relative(&self) -> &Box<T> {
        return &self.relative;
    }

    /// Retrieves the position and size of the widget relative to its root viewport
    pub fn get_absolute(&self) -> &Box<T> {
        return &self.absolute;
    }
}
