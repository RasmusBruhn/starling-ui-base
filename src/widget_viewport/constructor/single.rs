use crate::{Coord, ViewportBuilder, ViewportConstructor, geometry};

/// Constructs a single viewport geometry for a widget where the viewport
/// fills the entire widget
///
/// # Parameters
///
/// builder: The widget builder for populating the viewport
pub fn single<T: Coord>(builder: ViewportBuilder<T>) -> ViewportConstructor<T> {
    let geometry = geometry::Constant::new_full();

    return vec![(builder, geometry)];
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{GeometryInfo, PhysicalGeometry, Point, Rect, ViewportBuilderTrait, Widget};

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
    fn construct() {
        let viewport = Rect {
            ll: Point { x: 15.0, y: 30.0 },
            ur: Point { x: 35.0, y: 40.0 },
        };
        let viewport_inner = Rect {
            ll: Point { x: 17.0, y: 32.0 },
            ur: Point { x: 33.0, y: 38.0 },
        };
        let viewports = single(TestBuilder::new());
        let geometry = geometry::Constant::new_centered(&Point { x: 0.8, y: 0.6 });
        let info = GeometryInfo::without_sibling(viewport.get_size());
        let widget = Widget::new(geometry, viewports, &info, &viewport);

        let (result_viewport_geometry, result_widget_geometry) = widget
            .viewports
            .viewports
            .iter()
            .map(|viewport| {
                let viewport_geometry = *viewport.get_geometry();
                let widget_geometry = viewport
                    .iter()
                    .map(|widget| *widget.get_geometry())
                    .collect::<Vec<_>>();

                return (viewport_geometry, widget_geometry);
            })
            .collect::<(Vec<_>, Vec<_>)>();

        let correct_viewport_geometry = vec![PhysicalGeometry::from_parent(
            Rect {
                ll: Point { x: 0.0, y: 0.0 },
                ur: Point { x: 1.0, y: 1.0 },
            },
            &viewport_inner,
        )];
        let correct_widget_geometry = vec![vec![PhysicalGeometry::from_parent(
            Rect {
                ll: Point { x: 0.2, y: 0.4 },
                ur: Point { x: 0.8, y: 0.6 },
            },
            &viewport_inner,
        )]];

        assert_eq!(result_viewport_geometry, correct_viewport_geometry);
        assert_eq!(result_widget_geometry, correct_widget_geometry);
    }
}
