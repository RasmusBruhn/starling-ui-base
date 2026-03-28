use crate::Coord;

mod generator;
mod physical;

pub use generator::{WidgetGeometryGenerator, WidgetGeometryInfo};
pub use physical::WidgetPhysicalGeometry;

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
    pub fn new(
        generator: Box<dyn WidgetGeometryGenerator<T>>,
        info: &WidgetGeometryInfo<T>,
    ) -> Self {
        let geometry_relative = generator.generate(info);
        let physical = WidgetPhysicalGeometry::from_parent(geometry_relative, &info.viewport);

        return Self {
            physical,
            generator,
        };
    }

    /// Updates the physical geometry
    ///
    /// # Parameters
    ///
    /// info: The info of parent and sibling widgets used to update the geometry
    pub fn update(&mut self, info: &WidgetGeometryInfo<T>) {
        let geometry_relative = self.generator.generate(info);
        self.physical = WidgetPhysicalGeometry::from_parent(geometry_relative, info.viewport);
    }

    /// Retrieves the physical geometry
    pub fn get(&self) -> &WidgetPhysicalGeometry<T> {
        return &self.physical;
    }
}
