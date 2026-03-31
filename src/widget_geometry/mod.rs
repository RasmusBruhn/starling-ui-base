use crate::{Coord, Rect};

pub mod constructor;
mod generator;
mod info;
mod physical;
mod update_info;

pub use constructor as geometry;
pub use generator::{GeometryGenerator, GeometryGeneratorTrait};
pub use info::GeometryInfo;
pub use physical::PhysicalGeometry;
pub use update_info::GeometryUpdateStatus;

/// A description of the position and size of a widget and how to generate the
/// layout
#[derive(Debug)]
pub struct Geometry<T: Coord> {
    /// The position and size of the widget
    physical: PhysicalGeometry<T>,
    /// The generator for constructing the physical geometry
    generator: GeometryGenerator<T>,
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
    pub(crate) fn new(
        generator: GeometryGenerator<T>,
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
    ///
    /// force: If true then force update the geometry, otherwise only update if
    /// scheduled
    pub(crate) fn update(
        &mut self,
        info: &GeometryInfo<T>,
        viewport: &Rect<T>,
        force: bool,
    ) -> GeometryUpdateStatus {
        return if force {
            let pre_absolute_size = self.physical.absolute.get_size();
            let pre_relative = self.physical.relative;

            let relative = self.generator.generate(info);
            self.physical = PhysicalGeometry::from_parent(relative, viewport);

            GeometryUpdateStatus {
                relative: relative != pre_relative,
                absolute: self.physical.absolute.get_size() != pre_absolute_size,
                internal: false,
            }
        } else {
            GeometryUpdateStatus {
                relative: false,
                absolute: false,
                internal: false,
            }
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
        let generator = geometry::Constant::new_centered(&Point { x: 0.5, y: 0.8 });
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
        let generator = geometry::Constant::new_centered(&Point { x: 0.5, y: 0.8 });
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

        let result1 = input.update(&info, &viewport, true);
        let result2 = input.physical;

        let correct1 = GeometryUpdateStatus {
            relative: true,
            absolute: true,
            internal: false,
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
        let generator = geometry::Constant::new_centered(&Point { x: 0.5, y: 0.8 });
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

        let result1 = input.update(&info, &viewport, true);
        let result2 = input.physical;

        let correct1 = GeometryUpdateStatus {
            relative: true,
            absolute: false,
            internal: false,
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
        let generator = geometry::Constant::new_centered(&Point { x: 0.5, y: 0.8 });
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

        let result1 = input.update(&info, &viewport, true);
        let result2 = input.physical;

        let correct1 = GeometryUpdateStatus {
            relative: false,
            absolute: true,
            internal: false,
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
        let generator = geometry::Constant::new_centered(&Point { x: 0.5, y: 0.8 });
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

        let result1 = input.update(&info, &viewport, true);
        let result2 = input.physical;

        let correct1 = GeometryUpdateStatus {
            relative: false,
            absolute: false,
            internal: false,
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
    fn update_no_force() {
        let generator = geometry::Constant::new_centered(&Point { x: 0.5, y: 0.8 });
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

        let result1 = input.update(&info, &viewport, false);
        let result2 = input.physical;

        let correct1 = GeometryUpdateStatus {
            relative: false,
            absolute: false,
            internal: false,
        };
        let correct2 = PhysicalGeometry {
            relative: Rect {
                ll: Point { x: 0.0, y: 0.0 },
                ur: Point { x: 0.0, y: 0.0 },
            },
            absolute: Rect {
                ll: Point { x: 0.0, y: 0.0 },
                ur: Point { x: 0.0, y: 0.0 },
            },
        };

        assert_eq!(result1, correct1);
        assert_eq!(result2, correct2);
    }

    #[test]
    fn get() {
        let generator = geometry::Constant::new_centered(&Point { x: 0.5, y: 0.8 });
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
