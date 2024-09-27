use crate::models::line::Line;
use crate::models::model::Model;
use crate::models::point::Point;
use std::default::Default;

pub struct Resize {
    pub length_ab: f32,
    pub length_bc: f32,
    pub length_cd: f32,
    pub length_de: f32,
    pub length_ef: f32,
    pub length_fg: f32,
    pub length_gh: f32,
    pub length_ah: f32,

    pub radius_i: f32,
    pub radius_j: f32,
}

impl Default for Resize {
    fn default() -> Self {
        Self {
            length_ab: 110.0,
            length_bc: 37.0,
            length_cd: 35.0,
            length_de: 15.0,
            length_ef: 40.0,
            length_fg: 15.0,
            length_gh: 35.0,
            length_ah: 37.0,

            radius_i: 7.5,
            radius_j: 7.5,
        }
    }
}

impl Resize {
    pub fn update_values(&mut self, model: &mut Model) {
        if self.length_ab != model.length_ab {
            Self::update_points(
                &mut model.a,
                &mut model.b,
                self.length_ab,
                &mut model.length_ab,
            );
            self.sync_lengths(model);
        }
        if self.length_bc != model.length_bc {
            Self::update_points(
                &mut model.b,
                &mut model.c,
                self.length_bc,
                &mut model.length_bc,
            );
        }
        if self.length_cd != model.length_cd {
            Self::update_points(
                &mut model.c,
                &mut model.d,
                self.length_cd,
                &mut model.length_cd,
            );
            self.sync_lengths(model);
        }
        if self.length_de != model.length_de {
            Self::update_points(
                &mut model.d,
                &mut model.e,
                self.length_de,
                &mut model.length_de,
            );
            self.sync_lengths(model);
        }
        if self.length_ef != model.length_ef {
            Self::update_points(
                &mut model.e,
                &mut model.f,
                self.length_ef,
                &mut model.length_ef,
            );
            self.sync_lengths(model);
        }
        if self.length_fg != model.length_fg {
            Self::update_points(
                &mut model.f,
                &mut model.g,
                self.length_fg,
                &mut model.length_fg,
            );
            self.sync_lengths(model);
        }
        if self.length_gh != model.length_gh {
            Self::update_points(
                &mut model.g,
                &mut model.h,
                self.length_gh,
                &mut model.length_gh,
            );
            self.sync_lengths(model);
        }
        if self.length_ah != model.length_ah {
            Self::update_points(
                &mut model.a,
                &mut model.h,
                self.length_ah,
                &mut model.length_ah,
            );
            self.sync_lengths(model);
        }

        if self.radius_i != model.i_radius {
            model.i_radius = self.radius_i;
        }
        if self.radius_j != model.j_radius {
            model.j_radius = self.radius_j;
        }
    }

    fn update_points(
        start: &mut Point, end: &mut Point, new_length: f32, current_length: &mut f32,
    ) {
        let line = Self::resize_line(*start, *end, new_length, *current_length);
        *start = line.start;
        *end = line.end;
        *current_length = new_length;
    }

    fn resize_line(start: Point, end: Point, new_length: f32, current_length: f32) -> Line {
        let unit_vector = Point {
            x: end.x - start.x,
            y: end.y - start.y,
        };
        let magnitude = Point {
            x: unit_vector.x / current_length,
            y: unit_vector.y / current_length,
        };
        let midpoint = Self::midpoint(start, end);

        Self::new_line(magnitude, midpoint, new_length)
    }

    fn midpoint(start: Point, end: Point) -> Point {
        let x = (start.x + end.x) / 2.0;
        let y = (start.y + end.y) / 2.0;

        Point { x, y }
    }

    fn new_line(magnitude: Point, midpoint: Point, new_length: f32) -> Line {
        let new_start = Point {
            x: midpoint.x - (new_length / 2.0) * magnitude.x,
            y: midpoint.y - (new_length / 2.0) * magnitude.y,
        };
        let new_end = Point {
            x: midpoint.x + (new_length / 2.0) * magnitude.x,
            y: midpoint.y + (new_length / 2.0) * magnitude.y,
        };

        Line::new_plain(new_start, new_end)
    }

    fn sync_lengths(&mut self, model: &mut Model) {
        model.recalculate_lengths();

        self.length_ab = model.length_ab;
        self.length_bc = model.length_bc;
        self.length_cd = model.length_cd;
        self.length_de = model.length_de;
        self.length_ef = model.length_ef;
        self.length_fg = model.length_fg;
        self.length_gh = model.length_gh;
        self.length_ah = model.length_ah;
    }
}
