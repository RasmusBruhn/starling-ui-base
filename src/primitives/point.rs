use crate::Coord;
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// A single point in 2D-space
#[derive(Copy, Clone, Debug)]
pub struct Point<T: Coord> {
    /// The x-coordinate of the point
    pub x: T,
    /// The y-coordinate of the point
    pub y: T,
}

impl<T: Coord> Point<T> {
    /// Constructs a new point equal to (0, 0)
    #[cfg(test)]
    pub(crate) fn new_test() -> Self {
        return Self {
            x: T::from(0.0).unwrap(),
            y: T::from(0.0).unwrap(),
        };
    }
}

impl<T: Coord> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        return (self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y * other.y)
            < T::epsilon();
    }
}

impl<T: Coord> Neg for &Point<T> {
    type Output = Point<T>;

    fn neg(self) -> Self::Output {
        let x = -self.x;
        let y = -self.y;

        return Point { x, y };
    }
}

impl<T: Coord> Neg for Point<T> {
    type Output = Point<T>;

    fn neg(self) -> Self::Output {
        return -&self;
    }
}

impl<T: Coord> Add for &Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;

        return Point { x, y };
    }
}

impl<T: Coord> Add<Point<T>> for &Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Point<T>) -> Self::Output {
        return self + &rhs;
    }
}

impl<T: Coord> Add<&Point<T>> for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: &Point<T>) -> Self::Output {
        return &self + rhs;
    }
}

impl<T: Coord> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Point<T>) -> Self::Output {
        return &self + &rhs;
    }
}

impl<T: Coord> AddAssign<&Point<T>> for Point<T> {
    fn add_assign(&mut self, rhs: &Point<T>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<T: Coord> AddAssign for Point<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self += &rhs;
    }
}

impl<T: Coord> Sub for &Point<T> {
    type Output = Point<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;

        return Point { x, y };
    }
}

impl<T: Coord> Sub<Point<T>> for &Point<T> {
    type Output = Point<T>;

    fn sub(self, rhs: Point<T>) -> Self::Output {
        return self - &rhs;
    }
}

impl<T: Coord> Sub<&Point<T>> for Point<T> {
    type Output = Point<T>;

    fn sub(self, rhs: &Point<T>) -> Self::Output {
        return &self - rhs;
    }
}

impl<T: Coord> Sub for Point<T> {
    type Output = Point<T>;

    fn sub(self, rhs: Point<T>) -> Self::Output {
        return &self - &rhs;
    }
}

impl<T: Coord> SubAssign<&Point<T>> for Point<T> {
    fn sub_assign(&mut self, rhs: &Point<T>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl<T: Coord> SubAssign for Point<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self -= &rhs;
    }
}

impl<T: Coord> Mul for &Point<T> {
    type Output = Point<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;

        return Point { x, y };
    }
}

impl<T: Coord> Mul<Point<T>> for &Point<T> {
    type Output = Point<T>;

    fn mul(self, rhs: Point<T>) -> Self::Output {
        return self * &rhs;
    }
}

impl<T: Coord> Mul<&Point<T>> for Point<T> {
    type Output = Point<T>;

    fn mul(self, rhs: &Point<T>) -> Self::Output {
        return &self * rhs;
    }
}

impl<T: Coord> Mul for Point<T> {
    type Output = Point<T>;

    fn mul(self, rhs: Point<T>) -> Self::Output {
        return &self * &rhs;
    }
}

impl<T: Coord> MulAssign<&Point<T>> for Point<T> {
    fn mul_assign(&mut self, rhs: &Point<T>) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
    }
}

impl<T: Coord> MulAssign for Point<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self *= &rhs;
    }
}

impl<T: Coord> Div for &Point<T> {
    type Output = Point<T>;

    fn div(self, rhs: Self) -> Self::Output {
        let x = self.x / rhs.x;
        let y = self.y / rhs.y;

        return Point { x, y };
    }
}

impl<T: Coord> Div<Point<T>> for &Point<T> {
    type Output = Point<T>;

    fn div(self, rhs: Point<T>) -> Self::Output {
        return self / &rhs;
    }
}

impl<T: Coord> Div<&Point<T>> for Point<T> {
    type Output = Point<T>;

    fn div(self, rhs: &Point<T>) -> Self::Output {
        return &self / rhs;
    }
}

impl<T: Coord> Div for Point<T> {
    type Output = Point<T>;

    fn div(self, rhs: Point<T>) -> Self::Output {
        return &self / &rhs;
    }
}

impl<T: Coord> DivAssign<&Point<T>> for Point<T> {
    fn div_assign(&mut self, rhs: &Point<T>) {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
    }
}

impl<T: Coord> DivAssign for Point<T> {
    fn div_assign(&mut self, rhs: Self) {
        *self /= &rhs;
    }
}

impl<T: Coord> Mul<&T> for &Point<T> {
    type Output = Point<T>;

    fn mul(self, rhs: &T) -> Self::Output {
        let x = self.x * *rhs;
        let y = self.y * *rhs;

        return Point { x, y };
    }
}

impl<T: Coord> Mul<T> for &Point<T> {
    type Output = Point<T>;

    fn mul(self, rhs: T) -> Self::Output {
        return self * &rhs;
    }
}

impl<T: Coord> Mul<&T> for Point<T> {
    type Output = Point<T>;

    fn mul(self, rhs: &T) -> Self::Output {
        return &self * rhs;
    }
}

impl<T: Coord> Mul<T> for Point<T> {
    type Output = Point<T>;

    fn mul(self, rhs: T) -> Self::Output {
        return &self * &rhs;
    }
}

impl<T: Coord> MulAssign<&T> for Point<T> {
    fn mul_assign(&mut self, rhs: &T) {
        self.x = self.x * *rhs;
        self.y = self.y * *rhs;
    }
}

impl<T: Coord> MulAssign<T> for Point<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self *= &rhs;
    }
}

impl<T: Coord> Div<&T> for &Point<T> {
    type Output = Point<T>;

    fn div(self, rhs: &T) -> Self::Output {
        let x = self.x / *rhs;
        let y = self.y / *rhs;

        return Point { x, y };
    }
}

impl<T: Coord> Div<T> for &Point<T> {
    type Output = Point<T>;

    fn div(self, rhs: T) -> Self::Output {
        return self / &rhs;
    }
}

impl<T: Coord> Div<&T> for Point<T> {
    type Output = Point<T>;

    fn div(self, rhs: &T) -> Self::Output {
        return &self / rhs;
    }
}

impl<T: Coord> Div<T> for Point<T> {
    type Output = Point<T>;

    fn div(self, rhs: T) -> Self::Output {
        return &self / &rhs;
    }
}

impl<T: Coord> DivAssign<&T> for Point<T> {
    fn div_assign(&mut self, rhs: &T) {
        self.x = self.x / *rhs;
        self.y = self.y / *rhs;
    }
}

impl<T: Coord> DivAssign<T> for Point<T> {
    fn div_assign(&mut self, rhs: T) {
        *self /= &rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let result = Point { x: 1.0, y: 2.0 };

        let correct = Point { x: 1.0, y: 2.0 };

        assert_eq!(result, correct);
    }

    #[test]
    fn neg() {
        let input = Point { x: 1.0, y: 10.0 };

        let result1 = -input;
        let result2 = -&input;

        let correct = Point { x: -1.0, y: -10.0 };

        assert_eq!(result1, correct);
        assert_eq!(result2, correct);
    }

    #[test]
    fn add() {
        let input1 = Point { x: 6.0, y: 600.0 };
        let input2 = Point { x: 2.0, y: 20.0 };

        let result1 = input1 + input2;
        let result2 = &input1 + input2;
        let result3 = input1 + &input2;
        let result4 = &input1 + &input2;

        let correct = Point { x: 8.0, y: 620.0 };

        assert_eq!(result1, correct);
        assert_eq!(result2, correct);
        assert_eq!(result3, correct);
        assert_eq!(result4, correct);
    }

    #[test]
    fn add_assign() {
        let input1 = Point { x: 6.0, y: 600.0 };
        let input2 = Point { x: 2.0, y: 20.0 };

        let mut result1 = input1;
        let mut result2 = input1;

        result1 += input2;
        result2 += &input2;

        let correct = Point { x: 8.0, y: 620.0 };

        assert_eq!(result1, correct);
        assert_eq!(result2, correct);
    }

    #[test]
    fn sub() {
        let input1 = Point { x: 6.0, y: 600.0 };
        let input2 = Point { x: 2.0, y: 20.0 };

        let result1 = input1 - input2;
        let result2 = &input1 - input2;
        let result3 = input1 - &input2;
        let result4 = &input1 - &input2;

        let correct = Point { x: 4.0, y: 580.0 };

        assert_eq!(result1, correct);
        assert_eq!(result2, correct);
        assert_eq!(result3, correct);
        assert_eq!(result4, correct);
    }

    #[test]
    fn sub_assign() {
        let input1 = Point { x: 6.0, y: 600.0 };
        let input2 = Point { x: 2.0, y: 20.0 };

        let mut result1 = input1;
        let mut result2 = input1;

        result1 -= input2;
        result2 -= &input2;

        let correct = Point { x: 4.0, y: 580.0 };

        assert_eq!(result1, correct);
        assert_eq!(result2, correct);
    }

    #[test]
    fn mul() {
        let input1 = Point { x: 6.0, y: 600.0 };
        let input2 = Point { x: 2.0, y: 20.0 };

        let result1 = input1 * input2;
        let result2 = &input1 * input2;
        let result3 = input1 * &input2;
        let result4 = &input1 * &input2;

        let correct = Point {
            x: 12.0,
            y: 12000.0,
        };

        assert_eq!(result1, correct);
        assert_eq!(result2, correct);
        assert_eq!(result3, correct);
        assert_eq!(result4, correct);
    }

    #[test]
    fn mul_assign() {
        let input1 = Point { x: 6.0, y: 600.0 };
        let input2 = Point { x: 2.0, y: 20.0 };

        let mut result1 = input1;
        let mut result2 = input1;

        result1 *= input2;
        result2 *= &input2;

        let correct = Point {
            x: 12.0,
            y: 12000.0,
        };

        assert_eq!(result1, correct);
        assert_eq!(result2, correct);
    }

    #[test]
    fn div() {
        let input1 = Point { x: 6.0, y: 600.0 };
        let input2 = Point { x: 2.0, y: 20.0 };

        let result1 = input1 / input2;
        let result2 = &input1 / input2;
        let result3 = input1 / &input2;
        let result4 = &input1 / &input2;

        let correct = Point { x: 3.0, y: 30.0 };

        assert_eq!(result1, correct);
        assert_eq!(result2, correct);
        assert_eq!(result3, correct);
        assert_eq!(result4, correct);
    }

    #[test]
    fn div_assign() {
        let input1 = Point { x: 6.0, y: 600.0 };
        let input2 = Point { x: 2.0, y: 20.0 };

        let mut result1 = input1;
        let mut result2 = input1;

        result1 /= input2;
        result2 /= &input2;

        let correct = Point { x: 3.0, y: 30.0 };

        assert_eq!(result1, correct);
        assert_eq!(result2, correct);
    }

    #[test]
    fn mul_scalar() {
        let input1 = Point { x: 6.0, y: 600.0 };
        let input2 = 2.0;

        let result1 = input1 * input2;
        let result2 = &input1 * input2;
        let result3 = input1 * &input2;
        let result4 = &input1 * &input2;

        let correct = Point { x: 12.0, y: 1200.0 };

        assert_eq!(result1, correct);
        assert_eq!(result2, correct);
        assert_eq!(result3, correct);
        assert_eq!(result4, correct);
    }

    #[test]
    fn mul_assign_scalar() {
        let input1 = Point { x: 6.0, y: 600.0 };
        let input2 = 2.0;

        let mut result1 = input1;
        let mut result2 = input1;

        result1 *= input2;
        result2 *= &input2;

        let correct = Point { x: 12.0, y: 1200.0 };

        assert_eq!(result1, correct);
        assert_eq!(result2, correct);
    }

    #[test]
    fn div_scalar() {
        let input1 = Point { x: 6.0, y: 600.0 };
        let input2 = 2.0;

        let result1 = input1 / input2;
        let result2 = &input1 / input2;
        let result3 = input1 / &input2;
        let result4 = &input1 / &input2;

        let correct = Point { x: 3.0, y: 300.0 };

        assert_eq!(result1, correct);
        assert_eq!(result2, correct);
        assert_eq!(result3, correct);
        assert_eq!(result4, correct);
    }

    #[test]
    fn div_assign_scalar() {
        let input1 = Point { x: 6.0, y: 600.0 };
        let input2 = 2.0;

        let mut result1 = input1;
        let mut result2 = input1;

        result1 /= input2;
        result2 /= &input2;

        let correct = Point { x: 3.0, y: 300.0 };

        assert_eq!(result1, correct);
        assert_eq!(result2, correct);
    }
}
