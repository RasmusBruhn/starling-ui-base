use crate::Coord;

mod generator;
pub mod geometry;
mod physical;

pub use generator::{GeometryGenerator, GeometryInfo};
pub use physical::PhysicalGeometry;

use crate::Rect;

/// Status when updating the widget coordinates to check if something was
/// changed
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GeometryUpdateStatus {
    /// True if the relative WidgetBox of the widget changed
    relative: bool,
    /// True if the size of the absolute WidgetBox of the widget changed
    absolute: bool,
}

/// A description of the position and size of a widget and how to generate the
/// layout
#[derive(Debug)]
pub struct Geometry<T: Coord> {
    /// The position and size of the widget
    physical: PhysicalGeometry<T>,
    /// The generator for constructing the physical geometry
    generator: Box<dyn GeometryGenerator<T>>,
}

impl<T: Coord> Geometry<T> {
    /// Constructs a new widget geometry
    ///
    /// # Parameters
    ///
    /// generator: The generator used to construct the physical geometry
    ///
    /// info: The info of parent and sibling widgets used to set the geometry
    ///
    /// viewport: The absolute coordinates of the viewport for this widget
    pub fn new(
        generator: Box<dyn GeometryGenerator<T>>,
        info: &GeometryInfo<T>,
        viewport: &Rect<T>,
    ) -> Self {
        let geometry_relative = generator.generate(info);
        let physical = PhysicalGeometry::from_parent(geometry_relative, viewport);

        return Self {
            physical,
            generator,
        };
    }

    /// Updates the physical geometry, returns the update status
    ///
    /// # Parameters
    ///
    /// info: The info of parent and sibling widgets used to update the geometry
    ///
    /// viewport: The absolute coordinates of the viewport for this widget
    pub fn update(
        &mut self,
        info: &GeometryInfo<T>,
        viewport: &Rect<T>,
    ) -> GeometryUpdateStatus {
        let pre_absolute_size = self.physical.absolute.get_size();
        let pre_relative = self.physical.relative;

        let relative = self.generator.generate(info);
        self.physical = PhysicalGeometry::from_parent(relative, viewport);

        return GeometryUpdateStatus {
            relative: relative != pre_relative,
            absolute: self.physical.absolute.get_size() != pre_absolute_size,
        };
    }

    /// Retrieves the physical geometry
    pub fn get(&self) -> &PhysicalGeometry<T> {
        return &self.physical;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Point;

    #[test]
    fn new() {
        let generator = Box::new(geometry::Constant::new_centered(&Point { x: 0.5, y: 0.8 }));
        let viewport = Rect {
            ll: Point { x: 25.0, y: 5.0 },
            ur: Point { x: 45.0, y: 15.0 },
        };
        let info = GeometryInfo::without_sibling(viewport.get_size());

        let result = Geometry::new(generator, &info, &viewport).physical;

        let correct = PhysicalGeometry {
            relative: Rect {
                ll: Point { x: 0.25, y: 0.1 },
                ur: Point { x: 0.75, y: 0.9 },
            },
            absolute: Rect {
                ll: Point { x: 30.0, y: 6.0 },
                ur: Point { x: 40.0, y: 14.0 },
            },
        };

        assert_eq!(result, correct);
    }

    #[test]
    fn update() {
        let generator = Box::new(geometry::Constant::new_centered(&Point { x: 0.5, y: 0.8 }));
        let viewport = Rect {
            ll: Point { x: 25.0, y: 5.0 },
            ur: Point { x: 45.0, y: 15.0 },
        };
        let info = GeometryInfo::without_sibling(viewport.get_size());
        let mut input = Geometry {
            physical: PhysicalGeometry {
                relative: Rect {
                    ll: Point { x: 0.0, y: 0.0 },
                    ur: Point { x: 0.0, y: 0.0 },
                },
                absolute: Rect {
                    ll: Point { x: 0.0, y: 0.0 },
                    ur: Point { x: 0.0, y: 0.0 },
                },
            },
            generator,
        };

        let result1 = input.update(&info, &viewport);
        let result2 = input.physical;

        let correct1 = GeometryUpdateStatus {
            relative: true,
            absolute: true,
        };
        let correct2 = PhysicalGeometry {
            relative: Rect {
                ll: Point { x: 0.25, y: 0.1 },
                ur: Point { x: 0.75, y: 0.9 },
            },
            absolute: Rect {
                ll: Point { x: 30.0, y: 6.0 },
                ur: Point { x: 40.0, y: 14.0 },
            },
        };

        assert_eq!(result1, correct1);
        assert_eq!(result2, correct2);
    }

    #[test]
    fn update_relative() {
        let generator = Box::new(geometry::Constant::new_centered(&Point { x: 0.5, y: 0.8 }));
        let viewport = Rect {
            ll: Point { x: 25.0, y: 5.0 },
            ur: Point { x: 45.0, y: 15.0 },
        };
        let info = GeometryInfo::without_sibling(viewport.get_size());
        let mut input = Geometry {
            physical: PhysicalGeometry {
                relative: Rect {
                    ll: Point { x: 0.0, y: 0.0 },
                    ur: Point { x: 0.0, y: 0.0 },
                },
                absolute: Rect {
                    ll: Point { x: 30.0, y: 6.0 },
                    ur: Point { x: 40.0, y: 14.0 },
                },
            },
            generator,
        };

        let result1 = input.update(&info, &viewport);
        let result2 = input.physical;

        let correct1 = GeometryUpdateStatus {
            relative: true,
            absolute: false,
        };
        let correct2 = PhysicalGeometry {
            relative: Rect {
                ll: Point { x: 0.25, y: 0.1 },
                ur: Point { x: 0.75, y: 0.9 },
            },
            absolute: Rect {
                ll: Point { x: 30.0, y: 6.0 },
                ur: Point { x: 40.0, y: 14.0 },
            },
        };

        assert_eq!(result1, correct1);
        assert_eq!(result2, correct2);
    }

    #[test]
    fn update_absolute() {
        let generator = Box::new(geometry::Constant::new_centered(&Point { x: 0.5, y: 0.8 }));
        let viewport = Rect {
            ll: Point { x: 25.0, y: 5.0 },
            ur: Point { x: 45.0, y: 15.0 },
        };
        let info = GeometryInfo::without_sibling(viewport.get_size());
        let mut input = Geometry {
            physical: PhysicalGeometry {
                relative: Rect {
                    ll: Point { x: 0.25, y: 0.1 },
                    ur: Point { x: 0.75, y: 0.9 },
                },
                absolute: Rect {
                    ll: Point { x: 0.0, y: 0.0 },
                    ur: Point { x: 0.0, y: 0.0 },
                },
            },
            generator,
        };

        let result1 = input.update(&info, &viewport);
        let result2 = input.physical;

        let correct1 = GeometryUpdateStatus {
            relative: false,
            absolute: true,
        };
        let correct2 = PhysicalGeometry {
            relative: Rect {
                ll: Point { x: 0.25, y: 0.1 },
                ur: Point { x: 0.75, y: 0.9 },
            },
            absolute: Rect {
                ll: Point { x: 30.0, y: 6.0 },
                ur: Point { x: 40.0, y: 14.0 },
            },
        };

        assert_eq!(result1, correct1);
        assert_eq!(result2, correct2);
    }

    #[test]
    fn update_none() {
        let generator = Box::new(geometry::Constant::new_centered(&Point { x: 0.5, y: 0.8 }));
        let viewport = Rect {
            ll: Point { x: 25.0, y: 5.0 },
            ur: Point { x: 45.0, y: 15.0 },
        };
        let info = GeometryInfo::without_sibling(viewport.get_size());
        let mut input = Geometry {
            physical: PhysicalGeometry {
                relative: Rect {
                    ll: Point { x: 0.25, y: 0.1 },
                    ur: Point { x: 0.75, y: 0.9 },
                },
                absolute: Rect {
                    ll: Point { x: 30.0, y: 6.0 },
                    ur: Point { x: 40.0, y: 14.0 },
                },
            },
            generator,
        };

        let result1 = input.update(&info, &viewport);
        let result2 = input.physical;

        let correct1 = GeometryUpdateStatus {
            relative: false,
            absolute: false,
        };
        let correct2 = PhysicalGeometry {
            relative: Rect {
                ll: Point { x: 0.25, y: 0.1 },
                ur: Point { x: 0.75, y: 0.9 },
            },
            absolute: Rect {
                ll: Point { x: 30.0, y: 6.0 },
                ur: Point { x: 40.0, y: 14.0 },
            },
        };

        assert_eq!(result1, correct1);
        assert_eq!(result2, correct2);
    }

    #[test]
    fn get() {
        let generator = Box::new(geometry::Constant::new_centered(&Point { x: 0.5, y: 0.8 }));
        let viewport = Rect {
            ll: Point { x: 25.0, y: 5.0 },
            ur: Point { x: 45.0, y: 15.0 },
        };
        let info = GeometryInfo::without_sibling(viewport.get_size());
        let geometry = Geometry::new(generator, &info, &viewport);

        let result = geometry.get();

        let correct = PhysicalGeometry {
            relative: Rect {
                ll: Point { x: 0.25, y: 0.1 },
                ur: Point { x: 0.75, y: 0.9 },
            },
            absolute: Rect {
                ll: Point { x: 30.0, y: 6.0 },
                ur: Point { x: 40.0, y: 14.0 },
            },
        };

        assert_eq!(result, &correct);
    }
}
