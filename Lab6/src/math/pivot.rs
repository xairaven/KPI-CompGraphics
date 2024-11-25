use crate::geometry::point2d::Point2D;

pub fn calculate(points: &[Point2D]) -> Point2D {
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;

    for point in points {
        sum_x += point.x;
        sum_y += point.y;
    }

    let amount = points.len() as f32;
    Point2D::new(sum_x / amount, sum_y / amount)
}
