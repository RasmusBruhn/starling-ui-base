use crate::Coord;

mod generator;
pub mod geometry;
mod physical;

pub use generator::{WidgetGeometryGenerator, WidgetGeometryInfo};
pub use physical::WidgetPhysicalGeometry;

use crate::WidgetBox;

/// Status when updating the widget coordinates to check if something was
/// changed
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WidgetGeometryUpdateStatus {
    /// True if the relative WidgetBox of the widget changed
    relative: bool,
    /// True if the size of the absolute WidgetBox of the widget changed
    absolute: bool,
}

/// A description of the position and size of a widget and how to generate the
/// layout
#[derive(Debug)]
pub struct WidgetGeometry<T: Coord> {
    /// The position and size of the widget
    physical: WidgetPhysicalGeometry<T>,
    /// The generator for constructing the physical geometry
    generator: Box<dyn WidgetGeometryGenerator<T>>,
}

impl<T: Coord> WidgetGeometry<T> {
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
        generator: Box<dyn WidgetGeometryGenerator<T>>,
        info: &WidgetGeometryInfo<T>,
        viewport: &WidgetBox<T>,
    ) -> Self {
        let geometry_relative = generator.generate(info);
        let physical = WidgetPhysicalGeometry::from_parent(geometry_relative, viewport);

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
        info: &WidgetGeometryInfo<T>,
        viewport: &WidgetBox<T>,
    ) -> WidgetGeometryUpdateStatus {
        let pre_absolute_size = self.physical.absolute.get_size();
        let pre_relative = self.physical.relative;

        let relative = self.generator.generate(info);
        self.physical = WidgetPhysicalGeometry::from_parent(relative, viewport);

        return WidgetGeometryUpdateStatus {
            relative: relative != pre_relative,
            absolute: self.physical.absolute.get_size() != pre_absolute_size,
        };
    }

    /// Retrieves the physical geometry
    pub fn get(&self) -> &WidgetPhysicalGeometry<T> {
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
        let viewport = WidgetBox {
            ll: Point { x: 25.0, y: 5.0 },
            ur: Point { x: 45.0, y: 15.0 },
        };
        let info = WidgetGeometryInfo::without_sibling(viewport.get_size());

        let result = WidgetGeometry::new(generator, &info, &viewport).physical;

        let correct = WidgetPhysicalGeometry {
            relative: WidgetBox {
                ll: Point { x: 0.25, y: 0.1 },
                ur: Point { x: 0.75, y: 0.9 },
            },
            absolute: WidgetBox {
                ll: Point { x: 30.0, y: 6.0 },
                ur: Point { x: 40.0, y: 14.0 },
            },
        };

        assert_eq!(result, correct);
    }

    #[test]
    fn update() {
        let generator = Box::new(geometry::Constant::new_centered(&Point { x: 0.5, y: 0.8 }));
        let viewport = WidgetBox {
            ll: Point { x: 25.0, y: 5.0 },
            ur: Point { x: 45.0, y: 15.0 },
        };
        let info = WidgetGeometryInfo::without_sibling(viewport.get_size());

        let mut input = WidgetGeometry {
            physical: WidgetPhysicalGeometry {
                relative: WidgetBox {
                    ll: Point { x: 0.0, y: 0.0 },
                    ur: Point { x: 0.0, y: 0.0 },
                },
                absolute: WidgetBox {
                    ll: Point { x: 0.0, y: 0.0 },
                    ur: Point { x: 0.0, y: 0.0 },
                },
            },
            generator,
        };
        input.update(&info, &viewport);

        let result = input.physical;

        let correct = WidgetPhysicalGeometry {
            relative: WidgetBox {
                ll: Point { x: 0.25, y: 0.1 },
                ur: Point { x: 0.75, y: 0.9 },
            },
            absolute: WidgetBox {
                ll: Point { x: 30.0, y: 6.0 },
                ur: Point { x: 40.0, y: 14.0 },
            },
        };

        assert_eq!(result, correct);
    }
}
