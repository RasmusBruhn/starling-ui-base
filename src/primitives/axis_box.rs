use crate::{Coord, Point};

/// An axis-aligned box in 2D-space
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Box<T: Coord> {
    /// The position of the lower left corner of the box
    pub ll: Point<T>,
    /// The position of the upper right corner of the box
    pub ur: Point<T>,
}

impl<T: Coord> Box<T> {
    /// Constructs a new box
    ///
    /// # Parameters
    ///
    /// ll: The position of the lower left corner of the box
    ///
    /// ur: The position of the upper right corner of the box
    pub fn new(ll: Point<T>, ur: Point<T>) -> Self {
        return Self { ll, ur };
    }

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

        return Self::new(ll, ur);
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
    fn new() {
        let ll = Point::new(2.0, 20.0);
        let ur = Point::new(10.0, 30.0);

        let result = Box::new(ll, ur);

        let correct = Box {
            ll: Point::new(2.0, 20.0),
            ur: Point::new(10.0, 30.0),
        };

        assert_eq!(result, correct);
    }

    #[test]
    fn from_size() {
        let center = Point::new(6.0, 25.0);
        let size = Point::new(8.0, 10.0);

        let result = Box::from_size(&center, &size);

        let correct = Box::new(Point::new(2.0, 20.0), Point::new(10.0, 30.0));

        assert_eq!(result, correct);
    }

    #[test]
    fn get_center() {
        let input = Box::new(Point::new(2.0, 20.0), Point::new(10.0, 30.0));

        let result = input.get_center();

        let correct = Point::new(6.0, 25.0);

        assert_eq!(result, correct);
    }

    #[test]
    fn get_size() {
        let input = Box::new(Point::new(2.0, 20.0), Point::new(10.0, 30.0));

        let result = input.get_size();

        let correct = Point::new(8.0, 10.0);

        assert_eq!(result, correct);
    }
}
