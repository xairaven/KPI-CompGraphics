use crate::models::circle::Circle;
use crate::models::line::Line;
use crate::models::point::Point;

pub struct Projective {
    pub xx: f32,
    pub xy: f32,
    pub wx: f32,
    pub yx: f32,
    pub yy: f32,
    pub wy: f32,
    pub zero_x: f32,
    pub zero_y: f32,
    pub zero_w: f32,
}

impl Default for Projective {
    fn default() -> Self {
        Self {
            xx: 500.0,
            xy: 0.0,
            wx: 2.0,
            yx: 0.0,
            yy: 500.0,
            wy: 2.0,
            zero_x: 0.0,
            zero_y: 0.0,
            zero_w: 500.0,
        }
    }
}

impl Projective {
    pub fn is_projective_default(&self) -> bool {
        let default: Self = Projective::default();

        self.xx == default.xx
            && self.xy == default.xy
            && self.wx == default.wx
            && self.yx == default.yx
            && self.yy == default.yy
            && self.wy == default.wy
            && self.zero_x == default.zero_x
            && self.zero_y == default.zero_y
            && self.zero_w == default.zero_w
    }

    pub fn convert_line(&self, lines: Vec<Line>) -> Vec<Line> {
        if self.is_projective_default() {
            return lines;
        };

        lines
            .iter()
            .map(|line| {
                let start = self.convert_point(line.start);
                let end = self.convert_point(line.end);
                Line::new(start, end, line.stroke)
            })
            .collect()
    }

    pub fn convert_circle(&self, circle: Circle) -> Circle {
        if self.is_projective_default() {
            return circle;
        }

        let point = self.convert_point(Point::new(circle.center.x, circle.center.y));

        let radius = 2.5;

        Circle {
            center: Point::new(point.x, point.y),
            radius,
        }
    }

    fn convert_point(&self, point: Point) -> Point {
        let x =
            (self.zero_x * self.zero_w + self.xx * self.wx * point.x + self.xy * self.wy * point.y)
                / (self.zero_w + self.wx * point.x + self.wy * point.y);
        let y =
            (self.zero_y * self.zero_w + self.yx * self.wx * point.x + self.yy * self.wy * point.y)
                / (self.zero_w + self.wx * point.x + self.wy * point.y);

        Point::new(x, y)
    }
}
