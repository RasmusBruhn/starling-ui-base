use crate::{Coord, Point, Rect};
use std::time::Instant;

/// All information the genetator can use to construct the new geometry
#[derive(Clone, Debug, PartialEq)]
pub struct GeometryInfo<T: Coord> {
    /// The current time
    pub time: Instant,
    /// The size in absolute coordinates of the viewport this widget is inside
    pub viewport_size: Point<T>,
    /// The relative physical geometry of the previous sibling or None if this
    /// is the first widget in this viewport
    pub sibling: Option<Rect<T>>,
}

impl<T: Coord> GeometryInfo<T> {
    /// Constructs a new geometry info with a sibling
    ///
    /// # Parameters
    ///
    /// viewport_size: The size in absolute coordinates of the viewport this widget is inside
    ///
    /// sibling: The relative coordinates of the geometry of the previous
    /// sibling widget
    pub fn with_sibling(viewport_size: Point<T>, sibling: Rect<T>) -> Self {
        let time = Instant::now();

        return Self {
            time,
            viewport_size,
            sibling: Some(sibling),
        };
    }

    /// Constructs a new geometry info without a sibling
    ///
    /// # Parameters
    ///
    /// viewport_size: The size in absolute coordinates of the viewport this widget is inside
    pub fn without_sibling(viewport_size: Point<T>) -> Self {
        let time = Instant::now();

        return Self {
            time,
            viewport_size,
            sibling: None,
        };
    }

    /// Constructs a copy of the geometry info with a new sibling
    ///
    /// # Parameters
    ///
    /// sibling: The relative coordinates of the new sibling
    pub fn new_sibling(mut self, sibling: Rect<T>) -> Self {
        self.sibling = Some(sibling);
        return self;
    }

    /// Constructs a copy of the geometry info without the sibling
    pub fn remove_sibling(mut self) -> Self {
        self.sibling = None;
        return self;
    }

    /// Constructs a copy of the geometry with a new viewport size
    ///
    /// # Parameters
    ///
    /// viewport_size: The size in absolute coordinates of the new viewport
    pub fn new_viewport(mut self, viewport_size: Point<T>) -> Self {
        self.viewport_size = viewport_size;
        return self;
    }
}
