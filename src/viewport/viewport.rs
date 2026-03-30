use crate::{
    Coord, GeometryInfo, GeometryUpdateStatus, PhysicalGeometry, Rect, ViewportBuilder,
    ViewportManager, Widget,
};
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

/// A rectangular region inside a widget which can hold other widgets
#[derive(Debug)]
pub struct Viewport<T: Coord> {
    /// The geometry of this viewport
    geometry: PhysicalGeometry<T>,
    /// All widgets inside this viewport
    widgets: Vec<Rc<RefCell<Widget<T>>>>,
    /// The manager for this viewport
    manager: Box<dyn ViewportManager<T>>,
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
    /// manager: The manager for the viewport
    ///
    /// info: The info for building the geometry
    ///
    /// parent: The absolute coordinates of the parent widget geometry
    pub fn new(
        builder: Box<dyn ViewportBuilder<T>>,
        manager: Box<dyn ViewportManager<T>>,
        info: &GeometryInfo<T>,
        parent: &Rect<T>,
    ) -> Self {
        let geometry = PhysicalGeometry::from_parent(manager.update(info), parent);
        let widgets = builder
            .build(
                &info
                    .clone()
                    .remove_sibling()
                    .new_viewport(geometry.absolute.get_size()),
                &geometry.absolute,
            )
            .into_iter()
            .map(|widget| Rc::new(RefCell::new(widget)))
            .collect::<Vec<_>>();

        return Self {
            geometry,
            widgets,
            manager,
            builder,
        };
    }

    /// Updates the viewport and its widgets
    ///
    /// # Parameters
    ///
    /// new_geometry: True of the absolute size of the parent widget has changed
    ///
    /// info: The info for rebuilding the size, the sibling is guarenteed to be None
    ///
    /// parent: The absolute coordinates of the parent widget geometry
    pub fn update(
        new_geometry: bool,
        info: &GeometryInfo<T>,
        parent: &Rect<T>,
    ) -> GeometryUpdateStatus {
        todo!()
    }
}
