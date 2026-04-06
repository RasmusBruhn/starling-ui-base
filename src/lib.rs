//!
//! x-coordinate increases to the right while y-coordinate increases downwards.
//!
//! All viewports have an internal local coordinate system used for all children
//! with (0,0) at the upper left corner and (1,1) at the lower right corner
//!

mod primitives;
mod widget_geometry;
mod widget_viewport;

pub use primitives::{Coord, Point, Rect};
pub use widget_geometry::{
    Geometry, GeometryGenerator, GeometryGeneratorTrait, GeometryInfo, GeometryUpdateStatus,
    PhysicalGeometry, geometry,
};
use widget_viewport::ViewportList;
pub use widget_viewport::{
    Viewport, ViewportBuilder, ViewportBuilderTrait, ViewportConstructor, viewport,
};

/// A generic widget, the base of all elements in the ui
#[derive(Debug)]
pub struct Widget<T: Coord> {
    /// The description of the geometry of the widget
    geometry: Geometry<T>,
    /// All viewports for the widget
    viewports: ViewportList<T>,
}

impl<T: Coord> Widget<T> {
    /// Constructs a new widget
    ///
    /// # Parameters
    ///
    /// geometry: The generator used to construct the geometry
    ///
    /// viewports: The builders and managers for all the viewports
    ///
    /// info: The info of parent and sibling widgets used to set the geometry
    ///
    /// viewport: The absolute coordinates of the viewport for this widget
    pub fn new(
        geometry: GeometryGenerator<T>,
        viewports: ViewportConstructor<T>,
        info: &GeometryInfo<T>,
        viewport: &Rect<T>,
    ) -> Self {
        let geometry = Geometry::new(geometry, info, viewport);
        let viewports = ViewportList::new(
            viewports,
            &info
                .clone()
                .remove_sibling()
                .new_viewport(geometry.get().absolute.get_size()),
            &geometry.get().absolute,
        );

        return Self {
            geometry,
            viewports,
        };
    }

    /// Updates the widget and all its viewports
    ///
    /// # Parameters
    ///
    /// info: The geometry info used to update
    ///
    /// parent: The absolute coordinates of the parent viewport
    ///
    /// force: If true then force update the geometry, otherwise only update if
    /// scheduled
    pub fn update(
        &mut self,
        info: &GeometryInfo<T>,
        parent: &Rect<T>,
        force: bool,
    ) -> GeometryUpdateStatus {
        let mut status = self.geometry.update(info, parent, force);

        // Update viewports if size has changed
        if status.absolute {
            let info = info
                .clone()
                .new_viewport(self.geometry.get().absolute.get_size())
                .remove_sibling();

            status.internal |= self
                .viewports
                .update(&info, &self.geometry.get().absolute, status.absolute)
                .any();
        }

        return status;
    }

    /// Retrieves the current geometry of the widget
    pub fn get_geometry(&self) -> &PhysicalGeometry<T> {
        return self.geometry.get();
    }

    /// Constructs an iterator over all viewports
    pub fn iter(&self) -> impl Iterator<Item = &Viewport<T>> {
        return self.viewports.iter();
    }

    /// Constructs a new test widget with all geometries uninitialized to ((0,
    /// 0), (0, 0))
    ///
    /// # Parameters
    ///
    /// geometry: The generator used to construct the geometry
    ///
    /// viewports: The builders, managers, and widgets for all the viewports
    #[cfg(test)]
    pub(crate) fn new_test(
        geometry: GeometryGenerator<T>,
        viewports: Vec<(ViewportBuilder<T>, GeometryGenerator<T>, Vec<Widget<T>>)>,
    ) -> Self {
        let geometry = Geometry::new_test(geometry);
        let viewports = ViewportList::new_test(viewports);

        return Self {
            geometry,
            viewports,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestBuilder {}

    impl TestBuilder {
        fn new() -> Box<Self> {
            return Box::new(Self {});
        }
    }

    impl<T: Coord> ViewportBuilderTrait<T> for TestBuilder {
        fn build(&self, _info: &GeometryInfo<T>, _viewport: &Rect<T>) -> Vec<Widget<T>> {
            return Vec::new();
        }
    }

    #[test]
    fn new() {
        let generator = geometry::Constant::new_centered(&Point { x: 0.5, y: 0.8 });
        let viewport = Rect {
            ll: Point { x: 25.0, y: 5.0 },
            ur: Point { x: 45.0, y: 15.0 },
        };
        let viewports: ViewportConstructor<f64> = vec![(
            TestBuilder::new(),
            geometry::Constant::new_centered(&Point { x: 0.6, y: 0.2 }),
        )];
        let info = GeometryInfo::without_sibling(viewport.get_size());
        let widget = Widget::new(generator, viewports, &info, &viewport);

        let result_geometry = *widget.geometry.get();
        let result_viewports = widget
            .viewports
            .iter()
            .map(|viewport| *viewport.get_geometry())
            .collect::<Vec<_>>();

        let correct_geometry = PhysicalGeometry {
            relative: Rect {
                ll: Point { x: 0.25, y: 0.1 },
                ur: Point { x: 0.75, y: 0.9 },
            },
            absolute: Rect {
                ll: Point { x: 30.0, y: 6.0 },
                ur: Point { x: 40.0, y: 14.0 },
            },
        };
        let correct_viewports = vec![PhysicalGeometry {
            relative: Rect {
                ll: Point { x: 0.2, y: 0.4 },
                ur: Point { x: 0.8, y: 0.6 },
            },
            absolute: Rect {
                ll: Point { x: 32.0, y: 9.2 },
                ur: Point { x: 38.0, y: 10.8 },
            },
        }];

        assert_eq!(result_geometry, correct_geometry);
        assert_eq!(result_viewports, correct_viewports);
    }

    #[test]
    fn update_all() {
        let generator = geometry::Constant::new_centered(&Point { x: 0.5, y: 0.8 });
        let viewport = Rect {
            ll: Point { x: 25.0, y: 5.0 },
            ur: Point { x: 45.0, y: 15.0 },
        };
        let viewports: Vec<(
            ViewportBuilder<f64>,
            GeometryGenerator<f64>,
            Vec<Widget<f64>>,
        )> = vec![(
            TestBuilder::new(),
            geometry::Constant::new_centered(&Point { x: 0.6, y: 0.2 }),
            Vec::new(),
        )];
        let info = GeometryInfo::without_sibling(viewport.get_size());
        let mut widget = Widget::new_test(generator, viewports);

        let result_status = widget.update(&info, &viewport, true);
        let result_geometry = *widget.geometry.get();
        let result_viewports = widget
            .viewports
            .iter()
            .map(|viewport| *viewport.get_geometry())
            .collect::<Vec<_>>();

        let correct_status = GeometryUpdateStatus {
            relative: true,
            absolute: true,
            internal: true,
        };
        let correct_geometry = PhysicalGeometry {
            relative: Rect {
                ll: Point { x: 0.25, y: 0.1 },
                ur: Point { x: 0.75, y: 0.9 },
            },
            absolute: Rect {
                ll: Point { x: 30.0, y: 6.0 },
                ur: Point { x: 40.0, y: 14.0 },
            },
        };
        let correct_viewports = vec![PhysicalGeometry {
            relative: Rect {
                ll: Point { x: 0.2, y: 0.4 },
                ur: Point { x: 0.8, y: 0.6 },
            },
            absolute: Rect {
                ll: Point { x: 32.0, y: 9.2 },
                ur: Point { x: 38.0, y: 10.8 },
            },
        }];

        assert_eq!(result_status, correct_status);
        assert_eq!(result_geometry, correct_geometry);
        assert_eq!(result_viewports, correct_viewports);
    }

    #[test]
    fn update_outer() {
        let generator = geometry::Constant::new_centered(&Point { x: 0.5, y: 0.8 });
        let viewport = Rect {
            ll: Point { x: 25.0, y: 5.0 },
            ur: Point { x: 45.0, y: 15.0 },
        };
        let viewports: ViewportConstructor<f64> = vec![(
            TestBuilder::new(),
            geometry::Constant::new_centered(&Point { x: 0.6, y: 0.2 }),
        )];
        let info = GeometryInfo::without_sibling(viewport.get_size());
        let parent_inner = Rect {
            ll: Point { x: 30.0, y: 6.0 },
            ur: Point { x: 40.0, y: 14.0 },
        };
        let info_inner = GeometryInfo::without_sibling(parent_inner.get_size());
        let mut widget = Widget {
            geometry: Geometry::new_test(generator),
            viewports: ViewportList::new(viewports, &info_inner, &parent_inner),
        };

        let result_status = widget.update(&info, &viewport, true);
        let result_geometry = *widget.geometry.get();
        let result_viewports = widget
            .viewports
            .iter()
            .map(|viewport| *viewport.get_geometry())
            .collect::<Vec<_>>();

        let correct_status = GeometryUpdateStatus {
            relative: true,
            absolute: true,
            internal: false,
        };
        let correct_geometry = PhysicalGeometry {
            relative: Rect {
                ll: Point { x: 0.25, y: 0.1 },
                ur: Point { x: 0.75, y: 0.9 },
            },
            absolute: Rect {
                ll: Point { x: 30.0, y: 6.0 },
                ur: Point { x: 40.0, y: 14.0 },
            },
        };
        let correct_viewports = vec![PhysicalGeometry {
            relative: Rect {
                ll: Point { x: 0.2, y: 0.4 },
                ur: Point { x: 0.8, y: 0.6 },
            },
            absolute: Rect {
                ll: Point { x: 32.0, y: 9.2 },
                ur: Point { x: 38.0, y: 10.8 },
            },
        }];

        assert_eq!(result_status, correct_status);
        assert_eq!(result_geometry, correct_geometry);
        assert_eq!(result_viewports, correct_viewports);
    }

    #[test]
    fn update_none() {
        let generator = geometry::Constant::new_centered(&Point { x: 0.5, y: 0.8 });
        let viewport = Rect {
            ll: Point { x: 25.0, y: 5.0 },
            ur: Point { x: 45.0, y: 15.0 },
        };
        let viewports: Vec<(
            ViewportBuilder<f64>,
            GeometryGenerator<f64>,
            Vec<Widget<f64>>,
        )> = vec![(
            TestBuilder::new(),
            geometry::Constant::new_centered(&Point { x: 0.6, y: 0.2 }),
            Vec::new(),
        )];
        let info = GeometryInfo::without_sibling(viewport.get_size());
        let mut widget = Widget {
            geometry: Geometry::new(generator, &info, &viewport),
            viewports: ViewportList::new_test(viewports),
        };

        let result_status = widget.update(&info, &viewport, true);
        let result_geometry = *widget.geometry.get();
        let result_viewports = widget
            .viewports
            .iter()
            .map(|viewport| *viewport.get_geometry())
            .collect::<Vec<_>>();

        let correct_status = GeometryUpdateStatus {
            relative: false,
            absolute: false,
            internal: false,
        };
        let correct_geometry = PhysicalGeometry {
            relative: Rect {
                ll: Point { x: 0.25, y: 0.1 },
                ur: Point { x: 0.75, y: 0.9 },
            },
            absolute: Rect {
                ll: Point { x: 30.0, y: 6.0 },
                ur: Point { x: 40.0, y: 14.0 },
            },
        };
        let correct_viewports = vec![PhysicalGeometry {
            relative: Rect {
                ll: Point { x: 0.0, y: 0.0 },
                ur: Point { x: 0.0, y: 0.0 },
            },
            absolute: Rect {
                ll: Point { x: 0.0, y: 0.0 },
                ur: Point { x: 0.0, y: 0.0 },
            },
        }];

        assert_eq!(result_status, correct_status);
        assert_eq!(result_geometry, correct_geometry);
        assert_eq!(result_viewports, correct_viewports);
    }

    #[test]
    fn get_geometry() {
        let generator = geometry::Constant::new_centered(&Point { x: 0.5, y: 0.8 });
        let viewport = Rect {
            ll: Point { x: 25.0, y: 5.0 },
            ur: Point { x: 45.0, y: 15.0 },
        };
        let info = GeometryInfo::without_sibling(viewport.get_size());
        let widget = Widget::new(generator, Vec::new(), &info, &viewport);

        let result = *widget.get_geometry();

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

        assert_eq!(result, correct);
    }

    #[test]
    fn iter() {
        let generator = geometry::Constant::new_centered(&Point { x: 0.5, y: 0.8 });
        let viewport = Rect {
            ll: Point { x: 25.0, y: 5.0 },
            ur: Point { x: 45.0, y: 15.0 },
        };
        let viewports: ViewportConstructor<f64> = vec![(
            TestBuilder::new(),
            geometry::Constant::new_centered(&Point { x: 0.6, y: 0.2 }),
        )];
        let info = GeometryInfo::without_sibling(viewport.get_size());
        let widget = Widget::new(generator, viewports, &info, &viewport);

        let result = widget
            .iter()
            .map(|viewport| *viewport.get_geometry())
            .collect::<Vec<_>>();

        let correct = vec![PhysicalGeometry {
            relative: Rect {
                ll: Point { x: 0.2, y: 0.4 },
                ur: Point { x: 0.8, y: 0.6 },
            },
            absolute: Rect {
                ll: Point { x: 32.0, y: 9.2 },
                ur: Point { x: 38.0, y: 10.8 },
            },
        }];

        assert_eq!(result, correct);
    }
}
