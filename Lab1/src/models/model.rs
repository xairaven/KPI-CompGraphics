use crate::math::angle::Angle;
use crate::models::line::Line;
use crate::models::point::Point;
use eframe::epaint::{Color32, Stroke};

pub struct Model {
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub d: Point,
    pub e: Point,
    pub f: Point,
    pub g: Point,
    pub h: Point,

    pub i: Point,
    pub j: Point,
    pub i_radius: f32,
    pub j_radius: f32,

    pub length_ab: f32,
    pub length_bc: f32,
    pub length_cd: f32,
    pub length_de: f32,
    pub length_ef: f32,
    pub length_fg: f32,
    pub length_gh: f32,
    pub length_ah: f32,

    pub stroke: Stroke,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            a: Point { x: 10.0, y: 120.0 },
            b: Point { x: 10.0, y: 10.0 },
            c: Point { x: 47.0, y: 10.0 },
            d: Point { x: 47.0, y: 45.0 },
            e: Point { x: 32.0, y: 45.0 },
            f: Point { x: 32.0, y: 85.0 },
            g: Point { x: 47.0, y: 85.0 },
            h: Point { x: 47.0, y: 120.0 },

            i: Point { x: 20.0, y: 20.0 },
            i_radius: 7.5,
            j: Point { x: 20.0, y: 110.0 },
            j_radius: 7.5,

            length_ab: 110.0,
            length_bc: 37.0,
            length_cd: 35.0,
            length_de: 15.0,
            length_ef: 40.0,
            length_fg: 15.0,
            length_gh: 35.0,
            length_ah: 37.0,
            stroke: Stroke::new(2.0, Color32::from_rgb(0, 0, 0)),
        }
    }
}

impl Model {
    pub fn lines(&self) -> Vec<Line> {
        let mut lines = self.sides();
        lines.append(&mut self.circles());

        lines
    }

    pub fn sides(&self) -> Vec<Line> {
        let points = [
            self.a, self.b, self.c, self.d, self.e, self.f, self.g, self.h, self.a,
        ];

        let lines: Vec<Line> = points
            .windows(2)
            .map(|pair| Line::new(pair[0], pair[1], self.stroke))
            .collect();

        lines
    }

    pub fn circles(&self) -> Vec<Line> {
        let mut circles = self.circle(self.i, self.i_radius);
        let mut another_circle = self.circle(self.j, self.j_radius);

        circles.append(&mut another_circle);
        circles
    }

    fn circle(&self, center: Point, radius: f32) -> Vec<Line> {
        let mut lines: Vec<Line> = Vec::with_capacity(360);

        for degree in 0..360 {
            let current_rad = Angle::from_degree(degree as f32).radian();
            let next_rad = Angle::from_degree((degree + 1) as f32).radian();

            let start = Point {
                x: radius * f32::cos(current_rad) + center.x,
                y: radius * f32::sin(current_rad) + center.y,
            };

            let end = Point {
                x: radius * f32::cos(next_rad) + center.x,
                y: radius * f32::sin(next_rad) + center.y,
            };

            lines.push(Line::new(start, end, self.stroke));
        }

        lines
    }

    pub fn points_mut(&mut self) -> Vec<&mut Point> {
        vec![
            &mut self.a,
            &mut self.b,
            &mut self.c,
            &mut self.d,
            &mut self.e,
            &mut self.f,
            &mut self.g,
            &mut self.h,
            &mut self.i,
            &mut self.j,
        ]
    }

    pub fn recalculate_lengths(&mut self) {
        self.length_ab = Line::new_plain(self.a, self.b).length();
        self.length_bc = Line::new_plain(self.b, self.c).length();
        self.length_cd = Line::new_plain(self.c, self.d).length();
        self.length_de = Line::new_plain(self.d, self.e).length();
        self.length_ef = Line::new_plain(self.e, self.f).length();
        self.length_fg = Line::new_plain(self.f, self.g).length();
        self.length_gh = Line::new_plain(self.g, self.h).length();
        self.length_ah = Line::new_plain(self.h, self.a).length();
    }
}
