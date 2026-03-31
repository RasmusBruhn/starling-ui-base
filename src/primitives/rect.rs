use crate::{Coord, Point};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// An axis-aligned box in 2D-space
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rect<T: Coord> {
    /// The position of the lower left corner of the box
    pub ll: Point<T>,
    /// The position of the upper right corner of the box
    pub ur: Point<T>,
}

impl<T: Coord> Rect<T> {
    /// Constructs a new box from its center and size
    ///
    /// # Parameters
    ///
    /// center: The center of the box
    ///
    /// size: The size of the box
    pub fn from_size(center: &Point<T>, size: &Point<T>) -> Self {
        let ll = center - size * T::from(0.5).unwrap();
        let ur = center + size * T::from(0.5).unwrap();

        return Self { ll, ur };
    }

    /// Retrieves the center of the box
    pub fn get_center(&self) -> Point<T> {
        let center = (self.ur + self.ll) * T::from(0.5).unwrap();

        return center;
    }

    /// Retrieves the size of the box
    pub fn get_size(&self) -> Point<T> {
        let size = self.ur - self.ll;

        return size;
    }

    /// Constructs a new test rect equal to ((0, 0), (0, 0))
    #[cfg(test)]
    pub(crate) fn new_test() -> Self {
        return Self {
            ll: Point::new_test(),
            ur: Point::new_test(),
        };
    }
}

impl<T: Coord> Add<&Point<T>> for &Rect<T> {
    type Output = Rect<T>;

    fn add(self, rhs: &Point<T>) -> Self::Output {
        let ll = self.ll + rhs;
        let ur = self.ur + rhs;

        return Rect { ll, ur };
    }
}

impl<T: Coord> Add<Point<T>> for &Rect<T> {
    type Output = Rect<T>;

    fn add(self, rhs: Point<T>) -> Self::Output {
        return self + &rhs;
    }
}

impl<T: Coord> Add<&Point<T>> for Rect<T> {
    type Output = Rect<T>;

    fn add(self, rhs: &Point<T>) -> Self::Output {
        return &self + rhs;
    }
}

impl<T: Coord> Add<Point<T>> for Rect<T> {
    type Output = Rect<T>;

    fn add(self, rhs: Point<T>) -> Self::Output {
        return &self + &rhs;
    }
}

impl<T: Coord> AddAssign<&Point<T>> for Rect<T> {
    fn add_assign(&mut self, rhs: &Point<T>) {
        self.ll += rhs;
        self.ur += rhs;
    }
}

impl<T: Coord> AddAssign<Point<T>> for Rect<T> {
    fn add_assign(&mut self, rhs: Point<T>) {
        *self += &rhs;
    }
}

impl<T: Coord> Sub<&Point<T>> for &Rect<T> {
    type Output = Rect<T>;

    fn sub(self, rhs: &Point<T>) -> Self::Output {
        let ll = self.ll - rhs;
        let ur = self.ur - rhs;

        return Rect { ll, ur };
    }
}

impl<T: Coord> Sub<Point<T>> for &Rect<T> {
    type Output = Rect<T>;

    fn sub(self, rhs: Point<T>) -> Self::Output {
        return self - &rhs;
    }
}

impl<T: Coord> Sub<&Point<T>> for Rect<T> {
    type Output = Rect<T>;

    fn sub(self, rhs: &Point<T>) -> Self::Output {
        return &self - rhs;
    }
}

impl<T: Coord> Sub<Point<T>> for Rect<T> {
    type Output = Rect<T>;

    fn sub(self, rhs: Point<T>) -> Self::Output {
        return &self - &rhs;
    }
}

impl<T: Coord> SubAssign<&Point<T>> for Rect<T> {
    fn sub_assign(&mut self, rhs: &Point<T>) {
        self.ll -= rhs;
        self.ur -= rhs;
    }
}

impl<T: Coord> SubAssign<Point<T>> for Rect<T> {
    fn sub_assign(&mut self, rhs: Point<T>) {
        *self -= &rhs;
    }
}

impl<T: Coord> Mul<&Point<T>> for &Rect<T> {
    type Output = Rect<T>;

    fn mul(self, rhs: &Point<T>) -> Self::Output {
        let ll = self.ll * rhs;
        let ur = self.ur * rhs;

        return Rect { ll, ur };
    }
}

impl<T: Coord> Mul<Point<T>> for &Rect<T> {
    type Output = Rect<T>;

    fn mul(self, rhs: Point<T>) -> Self::Output {
        return self * &rhs;
    }
}

impl<T: Coord> Mul<&Point<T>> for Rect<T> {
    type Output = Rect<T>;

    fn mul(self, rhs: &Point<T>) -> Self::Output {
        return &self * rhs;
    }
}

impl<T: Coord> Mul<Point<T>> for Rect<T> {
    type Output = Rect<T>;

    fn mul(self, rhs: Point<T>) -> Self::Output {
        return &self * &rhs;
    }
}

impl<T: Coord> MulAssign<&Point<T>> for Rect<T> {
    fn mul_assign(&mut self, rhs: &Point<T>) {
        self.ll *= rhs;
        self.ur *= rhs;
    }
}

impl<T: Coord> MulAssign<Point<T>> for Rect<T> {
    fn mul_assign(&mut self, rhs: Point<T>) {
        *self *= &rhs;
    }
}

impl<T: Coord> Div<&Point<T>> for &Rect<T> {
    type Output = Rect<T>;

    fn div(self, rhs: &Point<T>) -> Self::Output {
        let ll = self.ll / rhs;
        let ur = self.ur / rhs;

        return Rect { ll, ur };
    }
}

impl<T: Coord> Div<Point<T>> for &Rect<T> {
    type Output = Rect<T>;

    fn div(self, rhs: Point<T>) -> Self::Output {
        return self / &rhs;
    }
}

impl<T: Coord> Div<&Point<T>> for Rect<T> {
    type Output = Rect<T>;

    fn div(self, rhs: &Point<T>) -> Self::Output {
        return &self / rhs;
    }
}

impl<T: Coord> Div<Point<T>> for Rect<T> {
    type Output = Rect<T>;

    fn div(self, rhs: Point<T>) -> Self::Output {
        return &self / &rhs;
    }
}

impl<T: Coord> DivAssign<&Point<T>> for Rect<T> {
    fn div_assign(&mut self, rhs: &Point<T>) {
        self.ll /= rhs;
        self.ur /= rhs;
    }
}

impl<T: Coord> DivAssign<Point<T>> for Rect<T> {
    fn div_assign(&mut self, rhs: Point<T>) {
        *self /= &rhs;
    }
}

impl<T: Coord> Mul<&T> for &Rect<T> {
    type Output = Rect<T>;

    fn mul(self, rhs: &T) -> Self::Output {
        let ll = self.ll * rhs;
        let ur = self.ur * rhs;

        return Rect { ll, ur };
    }
}

impl<T: Coord> Mul<T> for &Rect<T> {
    type Output = Rect<T>;

    fn mul(self, rhs: T) -> Self::Output {
        return self * &rhs;
    }
}

impl<T: Coord> Mul<&T> for Rect<T> {
    type Output = Rect<T>;

    fn mul(self, rhs: &T) -> Self::Output {
        return &self * rhs;
    }
}

impl<T: Coord> Mul<T> for Rect<T> {
    type Output = Rect<T>;

    fn mul(self, rhs: T) -> Self::Output {
        return &self * &rhs;
    }
}

impl<T: Coord> MulAssign<&T> for Rect<T> {
    fn mul_assign(&mut self, rhs: &T) {
        self.ll *= rhs;
        self.ur *= rhs;
    }
}

impl<T: Coord> MulAssign<T> for Rect<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self *= &rhs;
    }
}

impl<T: Coord> Div<&T> for &Rect<T> {
    type Output = Rect<T>;

    fn div(self, rhs: &T) -> Self::Output {
        let ll = self.ll / rhs;
        let ur = self.ur / rhs;

        return Rect { ll, ur };
    }
}

impl<T: Coord> Div<T> for &Rect<T> {
    type Output = Rect<T>;

    fn div(self, rhs: T) -> Self::Output {
        return self / &rhs;
    }
}

impl<T: Coord> Div<&T> for Rect<T> {
    type Output = Rect<T>;

    fn div(self, rhs: &T) -> Self::Output {
        return &self / rhs;
    }
}

impl<T: Coord> Div<T> for Rect<T> {
    type Output = Rect<T>;

    fn div(self, rhs: T) -> Self::Output {
        return &self / &rhs;
    }
}

impl<T: Coord> DivAssign<&T> for Rect<T> {
    fn div_assign(&mut self, rhs: &T) {
        self.ll /= rhs;
        self.ur /= rhs;
    }
}

impl<T: Coord> DivAssign<T> for Rect<T> {
    fn div_assign(&mut self, rhs: T) {
        *self /= &rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_size() {
        let center = Point { x: 6.0, y: 25.0 };
        let size = Point { x: 8.0, y: 10.0 };

        let result = Rect::from_size(&center, &size);

        let correct = Rect {
            ll: Point { x: 2.0, y: 20.0 },
            ur: Point { x: 10.0, y: 30.0 },
        };

        assert_eq!(result, correct);
    }

    #[test]
    fn get_center() {
        let input = Rect {
            ll: Point { x: 2.0, y: 20.0 },
            ur: Point { x: 10.0, y: 30.0 },
        };

        let result = input.get_center();

        let correct = Point { x: 6.0, y: 25.0 };

        assert_eq!(result, correct);
    }

    #[test]
    fn get_size() {
        let input = Rect {
            ll: Point { x: 2.0, y: 20.0 },
            ur: Point { x: 10.0, y: 30.0 },
        };

        let result = input.get_size();

        let correct = Point { x: 8.0, y: 10.0 };

        assert_eq!(result, correct);
    }

    #[test]
    fn add() {
        let input1 = Rect {
            ll: Point { x: 6.0, y: 600.0 },
            ur: Point { x: 0.6, y: 60.0 },
        };
        let input2 = Point { x: 2.0, y: 20.0 };

        let result1 = input1 + input2;
        let result2 = &input1 + input2;
        let result3 = input1 + &input2;
        let result4 = &input1 + &input2;

        let correct = Rect {
            ll: Point { x: 8.0, y: 620.0 },
            ur: Point { x: 2.6, y: 80.0 },
        };

        assert_eq!(result1, correct);
        assert_eq!(result2, correct);
        assert_eq!(result3, correct);
        assert_eq!(result4, correct);
    }

    #[test]
    fn add_assign() {
        let input1 = Rect {
            ll: Point { x: 6.0, y: 600.0 },
            ur: Point { x: 0.6, y: 60.0 },
        };
        let input2 = Point { x: 2.0, y: 20.0 };

        let mut result1 = input1;
        let mut result2 = input1;

        result1 += input2;
        result2 += &input2;

        let correct = Rect {
            ll: Point { x: 8.0, y: 620.0 },
            ur: Point { x: 2.6, y: 80.0 },
        };

        assert_eq!(result1, correct);
        assert_eq!(result2, correct);
    }

    #[test]
    fn sub() {
        let input1 = Rect {
            ll: Point { x: 6.0, y: 600.0 },
            ur: Point { x: 0.6, y: 60.0 },
        };
        let input2 = Point { x: 2.0, y: 20.0 };

        let result1 = input1 - input2;
        let result2 = &input1 - input2;
        let result3 = input1 - &input2;
        let result4 = &input1 - &input2;

        let correct = Rect {
            ll: Point { x: 4.0, y: 580.0 },
            ur: Point { x: -1.4, y: 40.0 },
        };

        assert_eq!(result1, correct);
        assert_eq!(result2, correct);
        assert_eq!(result3, correct);
        assert_eq!(result4, correct);
    }

    #[test]
    fn sub_assign() {
        let input1 = Rect {
            ll: Point { x: 6.0, y: 600.0 },
            ur: Point { x: 0.6, y: 60.0 },
        };
        let input2 = Point { x: 2.0, y: 20.0 };

        let mut result1 = input1;
        let mut result2 = input1;

        result1 -= input2;
        result2 -= &input2;

        let correct = Rect {
            ll: Point { x: 4.0, y: 580.0 },
            ur: Point { x: -1.4, y: 40.0 },
        };

        assert_eq!(result1, correct);
        assert_eq!(result2, correct);
    }

    #[test]
    fn mul() {
        let input1 = Rect {
            ll: Point { x: 6.0, y: 600.0 },
            ur: Point { x: 0.6, y: 60.0 },
        };
        let input2 = Point { x: 2.0, y: 20.0 };

        let result1 = input1 * input2;
        let result2 = &input1 * input2;
        let result3 = input1 * &input2;
        let result4 = &input1 * &input2;

        let correct = Rect {
            ll: Point {
                x: 12.0,
                y: 12000.0,
            },
            ur: Point { x: 1.2, y: 1200.0 },
        };

        assert_eq!(result1, correct);
        assert_eq!(result2, correct);
        assert_eq!(result3, correct);
        assert_eq!(result4, correct);
    }

    #[test]
    fn mul_assign() {
        let input1 = Rect {
            ll: Point { x: 6.0, y: 600.0 },
            ur: Point { x: 0.6, y: 60.0 },
        };
        let input2 = Point { x: 2.0, y: 20.0 };

        let mut result1 = input1;
        let mut result2 = input1;

        result1 *= input2;
        result2 *= &input2;

        let correct = Rect {
            ll: Point {
                x: 12.0,
                y: 12000.0,
            },
            ur: Point { x: 1.2, y: 1200.0 },
        };

        assert_eq!(result1, correct);
        assert_eq!(result2, correct);
    }

    #[test]
    fn div() {
        let input1 = Rect {
            ll: Point { x: 6.0, y: 600.0 },
            ur: Point { x: 0.6, y: 60.0 },
        };
        let input2 = Point { x: 2.0, y: 20.0 };

        let result1 = input1 / input2;
        let result2 = &input1 / input2;
        let result3 = input1 / &input2;
        let result4 = &input1 / &input2;

        let correct = Rect {
            ll: Point { x: 3.0, y: 30.0 },
            ur: Point { x: 0.3, y: 3.0 },
        };

        assert_eq!(result1, correct);
        assert_eq!(result2, correct);
        assert_eq!(result3, correct);
        assert_eq!(result4, correct);
    }

    #[test]
    fn div_assign() {
        let input1 = Rect {
            ll: Point { x: 6.0, y: 600.0 },
            ur: Point { x: 0.6, y: 60.0 },
        };
        let input2 = Point { x: 2.0, y: 20.0 };

        let mut result1 = input1;
        let mut result2 = input1;

        result1 /= input2;
        result2 /= &input2;

        let correct = Rect {
            ll: Point { x: 3.0, y: 30.0 },
            ur: Point { x: 0.3, y: 3.0 },
        };

        assert_eq!(result1, correct);
        assert_eq!(result2, correct);
    }

    #[test]
    fn mul_scalar() {
        let input1 = Rect {
            ll: Point { x: 6.0, y: 600.0 },
            ur: Point { x: 0.6, y: 60.0 },
        };
        let input2 = 2.0;

        let result1 = input1 * input2;
        let result2 = &input1 * input2;
        let result3 = input1 * &input2;
        let result4 = &input1 * &input2;

        let correct = Rect {
            ll: Point { x: 12.0, y: 1200.0 },
            ur: Point { x: 1.2, y: 120.0 },
        };

        assert_eq!(result1, correct);
        assert_eq!(result2, correct);
        assert_eq!(result3, correct);
        assert_eq!(result4, correct);
    }

    #[test]
    fn mul_assign_scalar() {
        let input1 = Rect {
            ll: Point { x: 6.0, y: 600.0 },
            ur: Point { x: 0.6, y: 60.0 },
        };
        let input2 = 2.0;

        let mut result1 = input1;
        let mut result2 = input1;

        result1 *= input2;
        result2 *= &input2;

        let correct = Rect {
            ll: Point { x: 12.0, y: 1200.0 },
            ur: Point { x: 1.2, y: 120.0 },
        };

        assert_eq!(result1, correct);
        assert_eq!(result2, correct);
    }

    #[test]
    fn div_scalar() {
        let input1 = Rect {
            ll: Point { x: 6.0, y: 600.0 },
            ur: Point { x: 0.6, y: 60.0 },
        };
        let input2 = 2.0;

        let result1 = input1 / input2;
        let result2 = &input1 / input2;
        let result3 = input1 / &input2;
        let result4 = &input1 / &input2;

        let correct = Rect {
            ll: Point { x: 3.0, y: 300.0 },
            ur: Point { x: 0.3, y: 30.0 },
        };

        assert_eq!(result1, correct);
        assert_eq!(result2, correct);
        assert_eq!(result3, correct);
        assert_eq!(result4, correct);
    }

    #[test]
    fn div_assign_scalar() {
        let input1 = Rect {
            ll: Point { x: 6.0, y: 600.0 },
            ur: Point { x: 0.6, y: 60.0 },
        };
        let input2 = 2.0;

        let mut result1 = input1;
        let mut result2 = input1;

        result1 /= input2;
        result2 /= &input2;

        let correct = Rect {
            ll: Point { x: 3.0, y: 300.0 },
            ur: Point { x: 0.3, y: 30.0 },
        };

        assert_eq!(result1, correct);
        assert_eq!(result2, correct);
    }
}
