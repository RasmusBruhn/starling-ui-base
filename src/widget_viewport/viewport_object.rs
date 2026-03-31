use crate::{
    Coord, Geometry, GeometryGenerator, GeometryInfo, GeometryUpdateStatus, PhysicalGeometry, Rect,
    ViewportBuilder, Widget,
};
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

/// A rectangular region inside a widget which can hold other widgets
#[derive(Debug)]
pub(super) struct Viewport<T: Coord> {
    /// The geometry of this viewport
    geometry: Geometry<T>,
    /// All widgets inside this viewport
    widgets: Vec<Rc<RefCell<Widget<T>>>>,
    /// The builder for this viewport
    builder: ViewportBuilder<T>,
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
    pub fn new(
        builder: ViewportBuilder<T>,
        generator: GeometryGenerator<T>,
        info: &GeometryInfo<T>,
        parent: &Rect<T>,
    ) -> Self {
        let geometry = Geometry::new(generator, info, parent);
        let widgets = builder
            .build(
                &info
                    .clone()
                    .new_viewport(geometry.get().absolute.get_size()),
                &geometry.get().absolute,
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
    pub fn update(
        &mut self,
        info: &GeometryInfo<T>,
        parent: &Rect<T>,
        force: bool,
    ) -> GeometryUpdateStatus {
        let mut status = self.geometry.update(info, parent, force);

        // Update children
        let mut last_changed = false;
        let mut info = info
            .clone()
            .new_viewport(self.geometry.get().absolute.get_size());
        for mut widget in self.widgets.iter().map(|widget| widget.borrow_mut()) {
            // Only update if it is scheduled or something external has changed
            if last_changed || status.absolute {
                let widget_status = widget.update(&info, &self.geometry.get().absolute, true);
                status.internal |= widget_status.any();
                last_changed = status.relative;
            }

            // Update the sibling for the next widget
            info = info.new_sibling(widget.get_geometry().relative);
        }

        return status;
    }

    /// Retrieves the geometry of the viewport
    pub fn get_geometry(&self) -> &PhysicalGeometry<T> {
        return self.geometry.get();
    }

    /// Retrieves an iterator over all widgets in the viewport
    pub fn iter(&self) -> impl Iterator<Item = Ref<'_, Widget<T>>> {
        return self.widgets.iter().map(|widget| widget.borrow());
    }

    /// Constructs a new test viewport with all geometries uninitialized to ((0,
    /// 0), (0, 0))
    ///
    /// # Parameters
    ///
    /// builder: The builder for the viewport
    ///
    /// generator: The geometry generator for the viewport
    ///
    /// widgets: The widgets for the viewport
    #[cfg(test)]
    pub(crate) fn new_test(
        builder: ViewportBuilder<T>,
        generator: GeometryGenerator<T>,
        widgets: Vec<Widget<T>>,
    ) -> Self {
        let geometry = Geometry::new_test(generator);
        let widgets = widgets
            .into_iter()
            .map(|widget| Rc::new(RefCell::new(widget)))
            .collect::<Vec<_>>();

        return Self {
            geometry,
            widgets,
            builder,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Point, ViewportBuilderTrait, geometry};

    #[derive(Debug)]
    struct TestBuilder {}

    impl TestBuilder {
        fn new() -> Box<Self> {
            return Box::new(Self {});
        }
    }

    impl<T: Coord> ViewportBuilderTrait<T> for TestBuilder {
        fn build(&self, info: &GeometryInfo<T>, viewport: &Rect<T>) -> Vec<Widget<T>> {
            let widget = Widget::new(
                geometry::Constant::new_centered(&Point {
                    x: T::from(0.6).unwrap(),
                    y: T::from(0.2).unwrap(),
                }),
                Vec::new(),
                info,
                viewport,
            );

            return vec![widget];
        }
    }

    #[test]
    fn new() {
        let parent = Rect {
            ll: Point { x: 15.0, y: 30.0 },
            ur: Point { x: 35.0, y: 40.0 },
        };
        let info = GeometryInfo::without_sibling(parent.get_size());
        let viewport_rect = Rect {
            ll: Point { x: 17.0, y: 32.0 },
            ur: Point { x: 33.0, y: 38.0 },
        };
        let builder = TestBuilder::new();
        let generator = geometry::Constant::new_centered(&Point { x: 0.8, y: 0.6 });
        let viewport = Viewport::new(builder, generator, &info, &parent);

        let result_geometry = *viewport.get_geometry();
        let result_widgets = viewport
            .iter()
            .map(|widget| *widget.get_geometry())
            .collect::<Vec<_>>();

        let correct_geometry = PhysicalGeometry::from_parent(
            Rect {
                ll: Point { x: 0.1, y: 0.2 },
                ur: Point { x: 0.9, y: 0.8 },
            },
            &parent,
        );
        let correct_widgets = vec![PhysicalGeometry::from_parent(
            Rect {
                ll: Point { x: 0.2, y: 0.4 },
                ur: Point { x: 0.8, y: 0.6 },
            },
            &viewport_rect,
        )];

        assert_eq!(result_geometry, correct_geometry);
        assert_eq!(result_widgets, correct_widgets);
    }

    #[test]
    fn update_success() {
        let parent = Rect {
            ll: Point { x: 15.0, y: 30.0 },
            ur: Point { x: 35.0, y: 40.0 },
        };
        let info = GeometryInfo::without_sibling(parent.get_size());
        let viewport_rect = Rect {
            ll: Point { x: 17.0, y: 32.0 },
            ur: Point { x: 33.0, y: 38.0 },
        };
        let widget = Widget::new_test(
            geometry::Constant::new_centered(&Point { x: 0.6, y: 0.2 }),
            Vec::new(),
        );
        let mut viewport = Viewport::new_test(
            TestBuilder::new(),
            geometry::Constant::new_centered(&Point { x: 0.8, y: 0.6 }),
            vec![widget],
        );

        let result_status = viewport.update(&info, &parent, true);
        let result_viewport = *viewport.get_geometry();
        let result_widgets = viewport
            .iter()
            .map(|widget| *widget.get_geometry())
            .collect::<Vec<_>>();

        let correct_status = GeometryUpdateStatus {
            relative: true,
            absolute: true,
            internal: true,
        };
        let correct_viewport = PhysicalGeometry::from_parent(
            Rect {
                ll: Point { x: 0.1, y: 0.2 },
                ur: Point { x: 0.9, y: 0.8 },
            },
            &parent,
        );
        let correct_widgets: Vec<PhysicalGeometry<f64>> = vec![PhysicalGeometry::from_parent(
            Rect {
                ll: Point { x: 0.2, y: 0.4 },
                ur: Point { x: 0.8, y: 0.6 },
            },
            &viewport_rect,
        )];

        assert_eq!(result_status, correct_status);
        assert_eq!(result_viewport, correct_viewport);
        assert_eq!(result_widgets, correct_widgets);
    }

    #[test]
    fn update_outer() {
        let parent = Rect {
            ll: Point { x: 15.0, y: 30.0 },
            ur: Point { x: 35.0, y: 40.0 },
        };
        let info = GeometryInfo::without_sibling(parent.get_size());
        let viewport_rect = Rect {
            ll: Point { x: 17.0, y: 32.0 },
            ur: Point { x: 33.0, y: 38.0 },
        };
        let widget = Widget::new(
            geometry::Constant::new_centered(&Point { x: 0.6, y: 0.2 }),
            Vec::new(),
            &info,
            &viewport_rect,
        );
        let mut viewport = Viewport::new_test(
            TestBuilder::new(),
            geometry::Constant::new_centered(&Point { x: 0.8, y: 0.6 }),
            vec![widget],
        );

        let result_status = viewport.update(&info, &parent, true);
        let result_viewport = *viewport.get_geometry();
        let result_widgets = viewport
            .iter()
            .map(|widget| *widget.get_geometry())
            .collect::<Vec<_>>();

        let correct_status = GeometryUpdateStatus {
            relative: true,
            absolute: true,
            internal: false,
        };
        let correct_viewport = PhysicalGeometry::from_parent(
            Rect {
                ll: Point { x: 0.1, y: 0.2 },
                ur: Point { x: 0.9, y: 0.8 },
            },
            &parent,
        );
        let correct_widgets: Vec<PhysicalGeometry<f64>> = vec![PhysicalGeometry::from_parent(
            Rect {
                ll: Point { x: 0.2, y: 0.4 },
                ur: Point { x: 0.8, y: 0.6 },
            },
            &viewport_rect,
        )];

        assert_eq!(result_status, correct_status);
        assert_eq!(result_viewport, correct_viewport);
        assert_eq!(result_widgets, correct_widgets);
    }

    #[test]
    fn update_none() {
        let parent = Rect {
            ll: Point { x: 15.0, y: 30.0 },
            ur: Point { x: 35.0, y: 40.0 },
        };
        let info = GeometryInfo::without_sibling(parent.get_size());
        let widget = Widget::new_test(
            geometry::Constant::new_centered(&Point { x: 0.6, y: 0.2 }),
            Vec::new(),
        );
        let mut viewport = Viewport {
            geometry: Geometry::new(
                geometry::Constant::new_centered(&Point { x: 0.8, y: 0.6 }),
                &info,
                &parent,
            ),
            widgets: vec![Rc::new(RefCell::new(widget))],
            builder: TestBuilder::new(),
        };

        let result_status = viewport.update(&info, &parent, true);
        let result_viewport = *viewport.get_geometry();
        let result_widgets = viewport
            .iter()
            .map(|widget| *widget.get_geometry())
            .collect::<Vec<_>>();

        let correct_status = GeometryUpdateStatus {
            relative: false,
            absolute: false,
            internal: false,
        };
        let correct_viewport = PhysicalGeometry::from_parent(
            Rect {
                ll: Point { x: 0.1, y: 0.2 },
                ur: Point { x: 0.9, y: 0.8 },
            },
            &parent,
        );
        let correct_widgets: Vec<PhysicalGeometry<f64>> = vec![PhysicalGeometry::new_test()];

        assert_eq!(result_status, correct_status);
        assert_eq!(result_viewport, correct_viewport);
        assert_eq!(result_widgets, correct_widgets);
    }

    #[test]
    fn get_geometry() {
        let parent = Rect {
            ll: Point { x: 15.0, y: 30.0 },
            ur: Point { x: 35.0, y: 40.0 },
        };
        let info = GeometryInfo::without_sibling(parent.get_size());
        let viewport = Viewport {
            geometry: Geometry::new(
                geometry::Constant::new_centered(&Point { x: 0.8, y: 0.6 }),
                &info,
                &parent,
            ),
            widgets: Vec::new(),
            builder: TestBuilder::new(),
        };

        let result = *viewport.get_geometry();

        let correct = PhysicalGeometry::from_parent(
            Rect {
                ll: Point { x: 0.1, y: 0.2 },
                ur: Point { x: 0.9, y: 0.8 },
            },
            &parent,
        );

        assert_eq!(result, correct);
    }

    #[test]
    fn iter() {
        let parent = Rect {
            ll: Point { x: 15.0, y: 30.0 },
            ur: Point { x: 35.0, y: 40.0 },
        };
        let info = GeometryInfo::without_sibling(parent.get_size());
        let viewport_rect = Rect {
            ll: Point { x: 17.0, y: 32.0 },
            ur: Point { x: 33.0, y: 38.0 },
        };
        let widget = Widget::new(
            geometry::Constant::new_centered(&Point { x: 0.6, y: 0.2 }),
            Vec::new(),
            &info,
            &viewport_rect,
        );
        let viewport = Viewport {
            geometry: Geometry::new(
                geometry::Constant::new_centered(&Point { x: 0.8, y: 0.6 }),
                &info,
                &parent,
            ),
            widgets: vec![Rc::new(RefCell::new(widget))],
            builder: TestBuilder::new(),
        };

        let result = viewport
            .iter()
            .map(|widget| *widget.get_geometry())
            .collect::<Vec<_>>();

        let correct = vec![PhysicalGeometry::from_parent(
            Rect {
                ll: Point { x: 0.2, y: 0.4 },
                ur: Point { x: 0.8, y: 0.6 },
            },
            &viewport_rect,
        )];

        assert_eq!(result, correct);
    }
}
