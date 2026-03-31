use crate::{Coord, Rect};

/// A description of the position and size of a widget
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PhysicalGeometry<T: Coord> {
    /// The position and size of the widget relative to its parent viewport
    pub relative: Rect<T>,
    /// The position and size of the widget relative to its root viewport
    pub absolute: Rect<T>,
}

impl<T: Coord> PhysicalGeometry<T> {
    /// Constructs a new physical widget geometry using the absolute geometry of
    /// its parent viewport
    ///
    /// # Parameters
    ///
    /// relative: The position and size of the widget relative to its parent viewport
    ///
    /// parent: The position and size of the parent widget relative to its parent viewport
    pub fn from_parent(relative: Rect<T>, parent: &Rect<T>) -> Self {
        let absolute = relative * parent.get_size() + parent.ll;

        return Self { relative, absolute };
    }

    /// Constructs a new test physical geometry with all rects set to ((0, 0),
    /// (0, 0))
    #[cfg(test)]
    pub(crate) fn new_test() -> Self {
        return Self {
            relative: Rect::new_test(),
            absolute: Rect::new_test(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Point;

    #[test]
    fn from_parent() {
        let relative = Rect {
            ll: Point { x: 0.2, y: 0.3 },
            ur: Point { x: 0.5, y: 0.9 },
        };
        let parent = Rect {
            ll: Point { x: 50.0, y: 30.0 },
            ur: Point { x: 70.0, y: 40.0 },
        };
        let absolute = Rect {
            ll: Point { x: 54.0, y: 33.0 },
            ur: Point { x: 60.0, y: 39.0 },
        };

        let result = PhysicalGeometry::from_parent(relative, &parent);

        let correct = PhysicalGeometry { relative, absolute };

        assert_eq!(result, correct);
    }
}
