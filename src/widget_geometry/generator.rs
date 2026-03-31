use crate::{Coord, GeometryInfo, Rect};
use std::fmt::Debug;

/// A trait for an object to be able to generate the physical geometry for a
/// widget
pub trait GeometryGeneratorTrait<T: Coord>: Debug {
    /// Generates the new relative physical geometry of the widget
    ///
    /// # Parameters
    ///
    /// info: All the info related to other widgets and viewports used to
    /// construct the geometry
    fn generate(&self, info: &GeometryInfo<T>) -> Rect<T>;
}

pub type GeometryGenerator<T> = Box<dyn GeometryGeneratorTrait<T>>;

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
