use crate::{Coord, WidgetBox};

/// A description of the position and size of a widget
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WidgetPhysicalGeometry<T: Coord> {
    /// The position and size of the widget relative to its parent viewport
    pub relative: WidgetBox<T>,
    /// The position and size of the widget relative to its root viewport
    pub absolute: WidgetBox<T>,
}

impl<T: Coord> WidgetPhysicalGeometry<T> {
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
        let absolute = WidgetBox { ll, ur };

        return Self { relative, absolute };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Point;

    #[test]
    fn from_parent() {
        let relative = WidgetBox {
            ll: Point { x: 0.2, y: 0.3 },
            ur: Point { x: 0.5, y: 0.9 },
        };
        let parent = WidgetBox {
            ll: Point { x: 50.0, y: 30.0 },
            ur: Point { x: 70.0, y: 40.0 },
        };
        let absolute = WidgetBox {
            ll: Point { x: 54.0, y: 33.0 },
            ur: Point { x: 60.0, y: 39.0 },
        };

        let result = WidgetPhysicalGeometry::from_parent(relative, &parent);

        let correct = WidgetPhysicalGeometry { relative, absolute };

        assert_eq!(result, correct);
    }
}
