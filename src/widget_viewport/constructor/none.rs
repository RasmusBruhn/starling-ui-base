use crate::{Coord, ViewportConstructor};

/// Constructs a empty viewport geometry for a widget for widgets with no
/// children
pub fn none<T: Coord>() -> ViewportConstructor<T> {
    return Vec::new();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{GeometryInfo, Point, Rect, Widget, geometry};

    #[test]
    fn construct() {
        let viewport = Rect {
            ll: Point { x: 15.0, y: 30.0 },
            ur: Point { x: 35.0, y: 40.0 },
        };
        let viewports = none();
        let geometry = geometry::Constant::new_centered(&Point { x: 0.8, y: 0.6 });
        let info = GeometryInfo::without_sibling(viewport.get_size());
        let widget = Widget::new(geometry, viewports, true, &info, &viewport);

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

        let correct_viewport_geometry = Vec::new();
        let correct_widget_geometry = Vec::<Vec<_>>::new();

        assert_eq!(result_viewport_geometry, correct_viewport_geometry);
        assert_eq!(result_widget_geometry, correct_widget_geometry);
    }
}
