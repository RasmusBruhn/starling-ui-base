#[cfg(test)]
use crate::Widget;
use crate::{Coord, GeometryGenerator, GeometryInfo, GeometryUpdateStatus, Rect};

mod builder;
pub mod constructor;
mod viewport_object;

pub use builder::{ViewportBuilder, ViewportBuilderTrait};
pub use constructor as viewport;
pub use viewport_object::Viewport;

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

    /// Constructs an iterator over all viewports
    pub(crate) fn iter(&self) -> impl Iterator<Item = &Viewport<T>> {
        return self.viewports.iter();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PhysicalGeometry, Point, geometry};

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
                true,
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
        let viewports = ViewportList::new(vec![(builder, generator)], &info, &parent);

        let result_geometry = viewports
            .viewports
            .iter()
            .map(|viewport| *viewport.get_geometry())
            .collect::<Vec<_>>();
        let result_widgets = viewports
            .viewports
            .iter()
            .map(|viewport| {
                return viewport
                    .iter()
                    .map(|widget| *widget.get_geometry())
                    .collect::<Vec<_>>();
            })
            .collect::<Vec<_>>();

        let correct_geometry = vec![PhysicalGeometry::from_parent(
            Rect {
                ll: Point { x: 0.1, y: 0.2 },
                ur: Point { x: 0.9, y: 0.8 },
            },
            &parent,
        )];
        let correct_widgets = vec![vec![PhysicalGeometry::from_parent(
            Rect {
                ll: Point { x: 0.2, y: 0.4 },
                ur: Point { x: 0.8, y: 0.6 },
            },
            &viewport_rect,
        )]];

        assert_eq!(result_geometry, correct_geometry);
        assert_eq!(result_widgets, correct_widgets);
    }

    #[test]
    fn update() {
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
            true,
        );
        let mut viewports = ViewportList::new_test(vec![(
            TestBuilder::new(),
            geometry::Constant::new_centered(&Point { x: 0.8, y: 0.6 }),
            vec![widget],
        )]);

        let result_status = viewports.update(&info, &parent, true);
        let result_viewport = viewports
            .viewports
            .iter()
            .map(|viewport| *viewport.get_geometry())
            .collect::<Vec<_>>();
        let result_widgets = viewports
            .viewports
            .iter()
            .map(|viewport| {
                return viewport
                    .iter()
                    .map(|widget| *widget.get_geometry())
                    .collect::<Vec<_>>();
            })
            .collect::<Vec<_>>();

        let correct_status = GeometryUpdateStatus {
            relative: true,
            absolute: true,
            internal: true,
        };
        let correct_viewport = vec![PhysicalGeometry::from_parent(
            Rect {
                ll: Point { x: 0.1, y: 0.2 },
                ur: Point { x: 0.9, y: 0.8 },
            },
            &parent,
        )];
        let correct_widgets = vec![vec![PhysicalGeometry::from_parent(
            Rect {
                ll: Point { x: 0.2, y: 0.4 },
                ur: Point { x: 0.8, y: 0.6 },
            },
            &viewport_rect,
        )]];

        assert_eq!(result_status, correct_status);
        assert_eq!(result_viewport, correct_viewport);
        assert_eq!(result_widgets, correct_widgets);
    }

    #[test]
    fn iter() {
        let parent = Rect {
            ll: Point { x: 15.0, y: 30.0 },
            ur: Point { x: 35.0, y: 40.0 },
        };
        let info = GeometryInfo::without_sibling(parent.get_size());
        let viewports = ViewportList::new(
            vec![
                (
                    TestBuilder::new(),
                    geometry::Constant::new_centered(&Point { x: 0.8, y: 0.6 }),
                ),
                (
                    TestBuilder::new(),
                    geometry::Constant::new_centered(&Point { x: 0.6, y: 0.4 }),
                ),
            ],
            &info,
            &parent,
        );

        let result = viewports
            .iter()
            .map(|viewport| *viewport.get_geometry())
            .collect::<Vec<_>>();

        let correct = vec![
            PhysicalGeometry::from_parent(
                Rect {
                    ll: Point { x: 0.1, y: 0.2 },
                    ur: Point { x: 0.9, y: 0.8 },
                },
                &parent,
            ),
            PhysicalGeometry::from_parent(
                Rect {
                    ll: Point { x: 0.2, y: 0.3 },
                    ur: Point { x: 0.8, y: 0.7 },
                },
                &parent,
            ),
        ];

        assert_eq!(result, correct);
    }
}
