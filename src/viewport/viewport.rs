use crate::{
    Coord, Geometry, GeometryGenerator, GeometryInfo, GeometryUpdateStatus, PhysicalGeometry, Rect,
    ViewportBuilder, Widget,
};
use std::{cell::RefCell, rc::Rc};

/// A rectangular region inside a widget which can hold other widgets
#[derive(Debug)]
pub(super) struct Viewport<T: Coord> {
    /// The geometry of this viewport
    geometry: Geometry<T>,
    /// All widgets inside this viewport
    widgets: Vec<Rc<RefCell<Widget<T>>>>,
    /// The builder for this viewport
    builder: Box<dyn ViewportBuilder<T>>,
}

impl<T: Coord> Viewport<T> {
    /// Constructs a new viewport
    ///
    /// # Parameter
    ///
    /// builder: The builder for the viewport
    ///
    /// generator: The geometry generator for the viewport
    ///
    /// info: The info for building the geometry, sibling is guarenteed to be None
    ///
    /// parent: The absolute coordinates of the parent widget geometry
    pub(super) fn new(
        builder: Box<dyn ViewportBuilder<T>>,
        generator: Box<dyn GeometryGenerator<T>>,
        info: &GeometryInfo<T>,
        parent: &Rect<T>,
    ) -> Self {
        let geometry = Geometry::new(generator, info, parent);
        let widgets = builder
            .build(
                &info.clone().new_viewport(geometry.absolute.get_size()),
                &geometry.absolute,
            )
            .into_iter()
            .map(|widget| Rc::new(RefCell::new(widget)))
            .collect::<Vec<_>>();

        return Self {
            geometry,
            widgets,
            builder,
        };
    }

    /// Updates the viewport and its widgets
    ///
    /// # Parameters
    ///
    /// info: The info for rebuilding the size, the sibling is guarenteed to be
    /// None
    ///
    /// parent: The absolute coordinates of the parent widget geometry
    ///
    /// force: If true then it forces the viewport to update, otherwise only
    /// updates if it is scheduled
    pub(super) fn update(
        &mut self,
        info: &GeometryInfo<T>,
        parent: &Rect<T>,
        force: bool,
    ) -> GeometryUpdateStatus {
        let mut status = self.geometry.update(info, parent, force);

        // Update children
        let mut last_changed = false;
        let mut info = info.clone().new_viewport(self.geometry.absolute.get_size());
        for mut widget in self.widgets.iter().map(|widget| widget.borrow_mut()) {
            // Only update if it is scheduled or something external has changed
            if last_changed || status.absolute {
                let widget_status = widget.update(&info, &self.geometry.absolute, true);
                status.internal |= widget_status.any();
                last_changed = status.relative;
            }

            // Update the sibling for the next widget
            info = info.new_sibling(widget.get_geometry().relative);
        }

        return status;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new() {}
}
