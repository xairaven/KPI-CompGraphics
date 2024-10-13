use crate::models::line::Line;
use crate::models::point::Point;
use egui::Stroke;

fn unit_vector(a: Point, b: Point) -> Point {
    let difference = (a.x - b.x, a.y - b.y);
    let length = f32::sqrt(difference.0.powf(2.0) + difference.1.powf(2.0));
    Point::new(difference.0 / length, difference.1 / length)
}

pub fn line_with_center(center: Point, another_point: Point, length: f32, stroke: Stroke) -> Line {
    let unit = unit_vector(center, another_point);

    let half_length = length / 2.0;

    let start = Point::new(
        center.x - half_length * unit.x,
        center.y - half_length * unit.y,
    );
    let end = Point::new(
        center.x + half_length * unit.x,
        center.y + half_length * unit.y,
    );

    Line::new(start, end, stroke)
}
