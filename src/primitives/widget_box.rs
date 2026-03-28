use crate::{Coord, Point};

/// An axis-aligned box in 2D-space
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WidgetBox<T: Coord> {
    /// The position of the lower left corner of the box
    pub ll: Point<T>,
    /// The position of the upper right corner of the box
    pub ur: Point<T>,
}

impl<T: Coord> WidgetBox<T> {
    /// Constructs a new box from its center and size
    ///
    /// # Parameters
    ///
    /// center: The center of the box
    ///
    /// size: The size of the box
    pub fn from_size(center: &Point<T>, size: &Point<T>) -> Self {
        let ll = center - size * T::from_f64(0.5).unwrap();
        let ur = center + size * T::from_f64(0.5).unwrap();

        return Self { ll, ur };
    }

    /// Retrieves the center of the box
    pub fn get_center(&self) -> Point<T> {
        let center = (self.ur + self.ll) * T::from_f64(0.5).unwrap();

        return center;
    }

    /// Retrieves the size of the box
    pub fn get_size(&self) -> Point<T> {
        let size = self.ur - self.ll;

        return size;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_size() {
        let center = Point { x: 6.0, y: 25.0 };
        let size = Point { x: 8.0, y: 10.0 };

        let result = WidgetBox::from_size(&center, &size);

        let correct = WidgetBox {
            ll: Point { x: 2.0, y: 20.0 },
            ur: Point { x: 10.0, y: 30.0 },
        };

        assert_eq!(result, correct);
    }

    #[test]
    fn get_center() {
        let input = WidgetBox {
            ll: Point { x: 2.0, y: 20.0 },
            ur: Point { x: 10.0, y: 30.0 },
        };

        let result = input.get_center();

        let correct = Point { x: 6.0, y: 25.0 };

        assert_eq!(result, correct);
    }

    #[test]
    fn get_size() {
        let input = WidgetBox {
            ll: Point { x: 2.0, y: 20.0 },
            ur: Point { x: 10.0, y: 30.0 },
        };

        let result = input.get_size();

        let correct = Point { x: 8.0, y: 10.0 };

        assert_eq!(result, correct);
    }
}
