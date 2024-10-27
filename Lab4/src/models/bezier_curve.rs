use crate::geometry::line::Line;
use crate::geometry::point::Point;
use crate::math::vector;
use crate::traits::positionable::Positionable;
use egui::Stroke;

pub fn bezier_curve<T: Positionable>(
    lines: &mut Vec<Line<Point>>, stroke: &Stroke, step: f32, control_first: &T,
    control_second: &T, defining: &T,
) {
    debug_assert!(step >= 0.0);

    let mut points: Vec<Point> = vec![];

    let mut counter = 0.0;
    while counter <= 1.0001 {
        let point = formula(control_first, control_second, defining, counter);
        points.push(point);

        counter += step;
    }

    points.windows(2).for_each(|pair| {
        let line = Line::new(pair[0], pair[1], *stroke);
        lines.push(line);
    });
}

fn formula<T: Positionable>(control_first: &T, control_second: &T, defining: &T, u: f32) -> Point {
    debug_assert!((0.0..=1.0001).contains(&u));

    let x = control_first.x() * (1.0 - u).powf(2.0)
        + 2.0 * defining.x() * u * (1.0 - u)
        + control_second.x() * u.powf(2.0);

    let y = control_first.y() * (1.0 - u).powf(2.0)
        + 2.0 * defining.y() * u * (1.0 - u)
        + control_second.y() * u.powf(2.0);

    Point::new(x, y)
}

// Change - 1st or 2nd point. If changing second defining point, first (change = 1) have to change.
pub fn c1_continuity<T: Positionable>(
    defining_first: &mut T, control: &mut T, defining_second: &mut T, change: u32,
) {
    let alpha_1 = vector::length(T::new(
        2.0 * (control.x() - defining_first.x()),
        2.0 * (control.y() - defining_first.y()),
    ));
    let alpha_2 = vector::length(T::new(
        2.0 * (defining_second.x() - control.x()),
        2.0 * (defining_second.y() - control.y()),
    ));

    if change == 1 {
        let x = (-1.0 * alpha_1 * (defining_second.x() - control.x())) / alpha_2 + control.x();
        let y = (-1.0 * alpha_1 * (defining_second.y() - control.y())) / alpha_2 + control.y();

        defining_first.set_x(x);
        defining_first.set_y(y);
    } else if change == 2 {
        let x = (alpha_2 * (control.x() - defining_first.x())) / alpha_1 + control.x();
        let y = (alpha_2 * (control.y() - defining_first.y())) / alpha_1 + control.y();

        defining_second.set_x(x);
        defining_second.set_y(y);
    }
}
