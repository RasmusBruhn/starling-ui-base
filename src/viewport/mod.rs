use crate::{Coord, GeometryInfo, Rect};

mod builder;
mod manager;
mod viewport;

pub use builder::ViewportBuilder;
pub use manager::ViewportManager;
use viewport::Viewport;

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
    /// data: The builders and managers for all the viewports to construct
    ///
    /// info: The info for building for viewport geometries, sibling is
    /// guarenteed to be None
    ///
    /// parent: The absolute coordinates of the parent geometry
    pub(crate) fn new(
        data: Vec<(Box<dyn ViewportBuilder<T>>, Box<dyn ViewportManager<T>>)>,
        info: &GeometryInfo<T>,
        parent: &Rect<T>,
    ) -> Self {
        let viewports = data
            .into_iter()
            .map(|(builder, manager)| Viewport::new(builder, manager, info, parent))
            .collect::<Vec<_>>();

        return Self { viewports };
    }
}
