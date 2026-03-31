use crate::{Coord, Point, Rect};
use std::{fmt::Debug, time::Instant};

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

        let result = GeometryInfo::with_sibling(viewport_size, sibling);

        let correct = GeometryInfo {
            time: result.time,
            viewport_size,
            sibling: Some(sibling),
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

    #[test]
    fn new_sibling() {
        let sibling = Rect {
            ll: Point { x: 0.2, y: 0.3 },
            ur: Point { x: 0.5, y: 0.9 },
        };
        let viewport_size = Point { x: 20.0, y: 10.0 };
        let info = GeometryInfo::without_sibling(viewport_size);

        let correct = GeometryInfo {
            time: info.time,
            viewport_size,
            sibling: Some(sibling),
        };

        let result = info.new_sibling(sibling);

        assert_eq!(result, correct);
    }

    #[test]
    fn remove_sibling() {
        let sibling = Rect {
            ll: Point { x: 0.2, y: 0.3 },
            ur: Point { x: 0.5, y: 0.9 },
        };
        let viewport_size = Point { x: 20.0, y: 10.0 };
        let info = GeometryInfo::with_sibling(viewport_size, sibling);

        let correct = GeometryInfo {
            time: info.time,
            viewport_size,
            sibling: None,
        };

        let result = info.remove_sibling();

        assert_eq!(result, correct);
    }

    #[test]
    fn new_viewport() {
        let sibling = Rect {
            ll: Point { x: 0.2, y: 0.3 },
            ur: Point { x: 0.5, y: 0.9 },
        };
        let viewport_size = Point { x: 20.0, y: 10.0 };
        let info = GeometryInfo::with_sibling(Point { x: 0.0, y: 0.0 }, sibling);

        let correct = GeometryInfo {
            time: info.time,
            viewport_size,
            sibling: Some(sibling),
        };

        let result = info.new_viewport(viewport_size);

        assert_eq!(result, correct);
    }
}
