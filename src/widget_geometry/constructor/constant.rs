use crate::{Coord, GeometryGeneratorTrait, Point, Rect};

/// A constant widget geometry which always has the same geometry, useful for
/// copying the viewport geometry
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constant<T: Coord> {
    /// The geometry of the widget
    pub geometry: Rect<T>,
}

impl<T: Coord> Constant<T> {
    /// Constructs a new constant widget geometry which copies the viewport
    /// geometry
    pub fn new_full() -> Box<Self> {
        return Box::new(Self {
            geometry: Rect {
                ll: Point {
                    x: T::from(0.0).unwrap(),
                    y: T::from(0.0).unwrap(),
                },
                ur: Point {
                    x: T::from(1.0).unwrap(),
                    y: T::from(1.0).unwrap(),
                },
            },
        });
    }

    /// Constructs a new constant widget geometry centered in its viewport
    ///
    /// # Parameters
    ///
    /// size: The size of the widget
    pub fn new_centered(size: &Point<T>) -> Box<Self> {
        let ll = (Point {
            x: T::from(1.0).unwrap(),
            y: T::from(1.0).unwrap(),
        } - size)
            * T::from(0.5).unwrap();
        let ur = ll + size;

        return Box::new(Self {
            geometry: Rect { ll, ur },
        });
    }
}

impl<T: Coord> GeometryGeneratorTrait<T> for Constant<T> {
    fn generate(&self, _info: &crate::GeometryInfo<T>) -> Rect<T> {
        return self.geometry;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_full() {
        let result = Constant::new_full();

        let correct = Constant {
            geometry: Rect {
                ll: Point { x: 0.0, y: 0.0 },
                ur: Point { x: 1.0, y: 1.0 },
            },
        };

        assert_eq!(*result, correct);
    }

    #[test]
    fn new_centered() {
        let result1 = Constant::new_centered(&Point { x: 0.5, y: 0.5 });
        let result2 = Constant::new_centered(&Point { x: 1.0, y: 1.0 });
        let result3 = Constant::new_centered(&Point { x: 2.0, y: 2.0 });

        let correct1 = Constant {
            geometry: Rect {
                ll: Point { x: 0.25, y: 0.25 },
                ur: Point { x: 0.75, y: 0.75 },
            },
        };
        let correct2 = Constant {
            geometry: Rect {
                ll: Point { x: 0.0, y: 0.0 },
                ur: Point { x: 1.0, y: 1.0 },
            },
        };
        let correct3 = Constant {
            geometry: Rect {
                ll: Point { x: -0.5, y: -0.5 },
                ur: Point { x: 1.5, y: 1.5 },
            },
        };

        assert_eq!(*result1, correct1);
        assert_eq!(*result2, correct2);
        assert_eq!(*result3, correct3);
    }

    mod generator {
        use super::*;
        use crate::GeometryInfo;

        #[test]
        fn full() {
            let generator = Constant::new_full();
            let info = GeometryInfo::without_sibling(Point { x: 10.0, y: 20.0 });

            let result = generator.generate(&info);

            let correct = Rect {
                ll: Point { x: 0.0, y: 0.0 },
                ur: Point { x: 1.0, y: 1.0 },
            };

            assert_eq!(result, correct);
        }

        #[test]
        fn small() {
            let generator = Constant::new_centered(&Point { x: 0.5, y: 0.8 });
            let info = GeometryInfo::without_sibling(Point { x: 10.0, y: 20.0 });

            let result = generator.generate(&info);

            let correct = Rect {
                ll: Point { x: 0.25, y: 0.1 },
                ur: Point { x: 0.75, y: 0.9 },
            };

            assert_eq!(result, correct);
        }

        #[test]
        fn offset() {
            let generator = Constant {
                geometry: Rect {
                    ll: Point { x: 0.1, y: 0.2 },
                    ur: Point { x: 0.8, y: 0.6 },
                },
            };
            let info = GeometryInfo::without_sibling(Point { x: 10.0, y: 20.0 });

            let result = generator.generate(&info);

            let correct = Rect {
                ll: Point { x: 0.1, y: 0.2 },
                ur: Point { x: 0.8, y: 0.6 },
            };

            assert_eq!(result, correct);
        }
    }
}
