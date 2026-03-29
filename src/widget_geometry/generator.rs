use crate::{Coord, Point, Rect};
use std::{fmt::Debug, time::Instant};

/// All information the genetator can use to construct the new geometry
#[derive(Clone, Debug, PartialEq)]
pub struct GeometryInfo<'a, T: Coord> {
    /// The current time
    pub time: Instant,
    /// The size in absolute coordinates of the viewport this widget is inside
    pub viewport_size: Point<T>,
    /// The relative physical geometry of the previous sibling or None if this
    /// is the first widget in this viewport
    pub sibling: Option<&'a Rect<T>>,
}

impl<'a, T: Coord> GeometryInfo<'a, T> {
    /// Constructs a new geometry info with a sibling
    ///
    /// # Parameters
    ///
    /// viewport_size: The size in absolute coordinates of the viewport this widget is inside
    ///
    /// sibling: The relative coordinates of the geometry of the previous
    /// sibling widget
    pub fn with_sibling(viewport_size: Point<T>, sibling: &'a Rect<T>) -> Self {
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
}

/// A trait for an object to be able to generate the physical geometry for a
/// widget
pub trait GeometryGenerator<T: Coord>: Debug {
    /// Generates the new relative physical geometry of the widget
    ///
    /// # Parameters
    ///
    /// info: All the info related to other widgets and viewports used to
    /// construct the geometry
    fn generate(&self, info: &GeometryInfo<T>) -> Rect<T>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Point;

    #[test]
    fn with_sibling() {
        let sibling = Rect {
            ll: Point { x: 0.2, y: 0.3 },
            ur: Point { x: 0.5, y: 0.9 },
        };
        let viewport_size = Point { x: 20.0, y: 10.0 };

        let result = GeometryInfo::with_sibling(viewport_size, &sibling);

        let correct = GeometryInfo {
            time: result.time,
            viewport_size,
            sibling: Some(&sibling),
        };

        assert_eq!(result, correct);
    }

    #[test]
    fn without_sibling() {
        let viewport_size = Point { x: 20.0, y: 10.0 };

        let result = GeometryInfo::without_sibling(viewport_size);

        let correct = GeometryInfo {
            time: result.time,
            viewport_size,
            sibling: None,
        };

        assert_eq!(result, correct);
    }
}
