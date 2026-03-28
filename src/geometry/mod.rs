use num_traits::Float;

mod generator;
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
pub struct WidgetGeometry<T: Float> {
    /// The position and size of the widget
    physical: WidgetPhysicalGeometry<T>,
    /// The generator for constructing the physical geometry
    generator: Box<dyn WidgetGeometryGenerator<T>>,
}

impl<T: Float> WidgetGeometry<T> {
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
