use crate::{Coord, Point, WidgetBox, WidgetGeometryGenerator};

/// A constant widget geometry which always has the same geometry, useful for
/// copying the viewport geometry
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constant<T: Coord> {
    /// The geometry of the widget
    pub geometry: WidgetBox<T>,
}

impl<T: Coord> Constant<T> {
    /// Constructs a new constant widget geometry which copies the viewport
    /// geometry
    pub fn new_full() -> Self {
        return Self {
            geometry: WidgetBox {
                ll: Point {
                    x: T::from(0.0).unwrap(),
                    y: T::from(0.0).unwrap(),
                },
                ur: Point {
                    x: T::from(1.0).unwrap(),
                    y: T::from(1.0).unwrap(),
                },
            },
        };
    }

    /// Constructs a new constant widget geometry centered in its viewport
    ///
    /// # Parameters
    ///
    /// size: The size of the widget
    pub fn new_centered(size: &Point<T>) -> Self {
        let ll = (Point {
            x: T::from(1.0).unwrap(),
            y: T::from(1.0).unwrap(),
        } - size)
            * T::from(0.5).unwrap();
        let ur = ll + size;

        return Self {
            geometry: WidgetBox { ll, ur },
        };
    }
}

impl<T: Coord> WidgetGeometryGenerator<T> for Constant<T> {
    fn generate(&self, _info: &crate::WidgetGeometryInfo<T>) -> WidgetBox<T> {
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
            geometry: WidgetBox {
                ll: Point { x: 0.0, y: 0.0 },
                ur: Point { x: 1.0, y: 1.0 },
            },
        };

        assert_eq!(result, correct);
    }

    #[test]
    fn new_centered() {
        let result1 = Constant::new_centered(&Point { x: 0.5, y: 0.5 });
        let result2 = Constant::new_centered(&Point { x: 1.0, y: 1.0 });
        let result3 = Constant::new_centered(&Point { x: 2.0, y: 2.0 });

        let correct1 = Constant {
            geometry: WidgetBox {
                ll: Point { x: 0.25, y: 0.25 },
                ur: Point { x: 0.75, y: 0.75 },
            },
        };
        let correct2 = Constant {
            geometry: WidgetBox {
                ll: Point { x: 0.0, y: 0.0 },
                ur: Point { x: 1.0, y: 1.0 },
            },
        };
        let correct3 = Constant {
            geometry: WidgetBox {
                ll: Point { x: -0.5, y: -0.5 },
                ur: Point { x: 1.5, y: 1.5 },
            },
        };

        assert_eq!(result1, correct1);
        assert_eq!(result2, correct2);
        assert_eq!(result3, correct3);
    }

    mod generator {
        use super::*;
        use crate::WidgetGeometryInfo;

        #[test]
        fn full() {
            let generator = Constant::new_full();
            let info = WidgetGeometryInfo::without_sibling(Point { x: 10.0, y: 20.0 });

            let result = generator.generate(&info);

            let correct = WidgetBox {
                ll: Point { x: 0.0, y: 0.0 },
                ur: Point { x: 1.0, y: 1.0 },
            };

            assert_eq!(result, correct);
        }

        #[test]
        fn small() {
            let generator = Constant::new_centered(&Point { x: 0.5, y: 0.5 });
            let info = WidgetGeometryInfo::without_sibling(Point { x: 10.0, y: 20.0 });

            let result = generator.generate(&info);

            let correct = WidgetBox {
                ll: Point { x: 0.25, y: 0.25 },
                ur: Point { x: 0.75, y: 0.75 },
            };

            assert_eq!(result, correct);
        }

        #[test]
        fn offset() {
            let generator = Constant {
                geometry: WidgetBox {
                    ll: Point { x: 0.1, y: 0.2 },
                    ur: Point { x: 0.8, y: 0.6 },
                },
            };
            let info = WidgetGeometryInfo::without_sibling(Point { x: 10.0, y: 20.0 });

            let result = generator.generate(&info);

            let correct = WidgetBox {
                ll: Point { x: 0.1, y: 0.2 },
                ur: Point { x: 0.8, y: 0.6 },
            };

            assert_eq!(result, correct);
        }
    }
}
