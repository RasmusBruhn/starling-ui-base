#[cfg(test)]
use crate::Widget;
use crate::{Coord, GeometryGenerator, GeometryInfo, GeometryUpdateStatus, Rect};

mod builder;
pub mod constructor;
mod viewport_object;

pub use builder::{ViewportBuilder, ViewportBuilderTrait};
pub use constructor as viewport;
use viewport_object::Viewport;

pub type ViewportConstructor<T> = Vec<(ViewportBuilder<T>, GeometryGenerator<T>)>;

/// A list of all viewports in a widget
#[derive(Debug)]
pub(crate) struct ViewportList<T: Coord> {
    /// The list of the viewports
    viewports: Vec<Viewport<T>>,
}

impl<T: Coord> ViewportList<T> {
    /// Constructs a new viewport list
    ///
    /// # Parameters
    ///
    /// viewports: The builders and managers for all the viewports to construct
    ///
    /// info: The info for building for viewport geometries, sibling is
    /// guarenteed to be None
    ///
    /// parent: The absolute coordinates of the parent geometry
    pub(crate) fn new(
        viewports: ViewportConstructor<T>,
        info: &GeometryInfo<T>,
        parent: &Rect<T>,
    ) -> Self {
        let viewports = viewports
            .into_iter()
            .map(|(builder, manager)| Viewport::new(builder, manager, info, parent))
            .collect::<Vec<_>>();

        return Self { viewports };
    }

    /// Updates all viewports of the widget
    ///
    /// # Parameters
    ///
    /// info: The info for updating the geometry, sibling is guarenteed to be None
    ///
    /// parent: The absolute coordinates of the parent widget geometry
    ///
    /// force: If true, force update all viewports, otherwise only update if scheduled
    pub(crate) fn update(
        &mut self,
        info: &GeometryInfo<T>,
        parent: &Rect<T>,
        force: bool,
    ) -> GeometryUpdateStatus {
        return self
            .viewports
            .iter_mut()
            .map(|viewport| {
                return if force {
                    viewport.update(info, parent, force)
                } else {
                    GeometryUpdateStatus::new(false)
                };
            })
            .fold(GeometryUpdateStatus::new(false), |a, b| a | b);
    }

    /// Constructs a new test viewport list with all geometries uninitialized to
    /// ((0, 0), (0, 0))
    ///
    /// # Parameters
    ///
    /// viewports: The builders, managers and widgets for all the viewports
    #[cfg(test)]
    pub(crate) fn new_test(
        viewports: Vec<(ViewportBuilder<T>, GeometryGenerator<T>, Vec<Widget<T>>)>,
    ) -> Self {
        let viewports = viewports
            .into_iter()
            .map(|(builder, generator, widgets)| Viewport::new_test(builder, generator, widgets))
            .collect::<Vec<_>>();

        return Self { viewports };
    }
}
