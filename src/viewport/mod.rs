use crate::{Coord, Widget, PhysicalGeometry};
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
}

impl<T: Coord> Viewport<T> {
    /// Constructs an iterator over all widgets in the viewport
    pub fn iter(&self) -> impl Iterator<Item = Ref<Widget<T>>> {
        return self.widgets.iter().map(|widget| return widget.borrow());
    }

    /// Constructs a mutable iterator over all widgets in the viewport
    pub fn iter_mut(&self) -> impl Iterator<Item = RefMut<Widget<T>>> {
        return self.widgets.iter().map(|widget| return widget.borrow_mut());
    }
}
