//!
//! x-coordinate increases to the right while y-coordinate increases downwards.
//!
//! All viewports have an internal local coordinate system used for all children
//! with (0,0) at the upper left corner and (1,1) at the lower right corner
//!

mod primitives;
mod viewport;
mod widget_geometry;

pub use primitives::{Coord, Point, Rect};
pub use viewport::Viewport;
pub use widget_geometry::{
    Geometry, GeometryGenerator, GeometryInfo, GeometryUpdateStatus, PhysicalGeometry, geometry,
};

/// A generic widget, the base of all elements in the ui
#[derive(Debug)]
pub struct Widget<T: Coord> {
    /// The description of the geometry of the widget
    geometry: Geometry<T>,
}

impl<T: Coord> Widget<T> {
    /// Constructs a new widget
    ///
    /// # Parameters
    ///
    /// geometry: The generator used to construct the geometry
    ///
    /// info: The info of parent and sibling widgets used to set the geometry
    ///
    /// viewport: The absolute coordinates of the viewport for this widget
    pub fn new(
        geometry: Box<dyn GeometryGenerator<T>>,
        info: &GeometryInfo<T>,
        viewport: &Rect<T>,
    ) -> Self {
        let geometry = Geometry::new(geometry, info, viewport);

        return Self { geometry };
    }

    /// Retrieves the current geometry of the widget
    pub fn get_geometry(&self) -> &PhysicalGeometry<T> {
        return self.geometry.get();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_geometry() {
        let generator = Box::new(geometry::Constant::new_centered(&Point { x: 0.5, y: 0.8 }));
        let viewport = Rect {
            ll: Point { x: 25.0, y: 5.0 },
            ur: Point { x: 45.0, y: 15.0 },
        };
        let info = GeometryInfo::without_sibling(viewport.get_size());
        let widget = Widget::new(generator, &info, &viewport);

        let result = widget.get_geometry();

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
