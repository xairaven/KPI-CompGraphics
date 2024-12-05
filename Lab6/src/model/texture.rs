use crate::errors::texture::TextureError;
use crate::geometry::line3d::Line3D;
use crate::geometry::point2d::Point2D;
use crate::geometry::point3d::Point3D;
use crate::graphics::screen::ScreenParams;
use crate::math::pivot;
use crate::model::surface::Surface;
use crate::ui::styles::{colors, strokes};
use egui::{Color32, Stroke};
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::num::ParseFloatError;
use std::path::PathBuf;

pub struct Texture {
    pub is_enabled: bool,

    pub display_delta_u: f32,
    pub display_delta_v: f32,
    pub display_angle: f32,

    pub scale_factor: f32,

    pub color_contour: Color32,

    is_loaded: bool,

    delta_u: f32,
    delta_v: f32,
    angle: f32,

    points: Vec<Point2D>,

    stroke: Stroke,
}

impl Default for Texture {
    fn default() -> Self {
        Self {
            is_enabled: false,
            is_loaded: false,

            display_delta_u: 0.0,
            display_delta_v: 0.0,
            display_angle: 0.0,

            scale_factor: 25.0,

            delta_u: 0.0,
            delta_v: 0.0,
            angle: 0.0,

            points: vec![],

            color_contour: colors::PINK,
            stroke: strokes::texture_pink(0.1),
        }
    }
}

impl Texture {
    pub fn generate(&mut self, radius: f32, screen: ScreenParams) -> Vec<Line3D> {
        if self.is_enabled {
            self.sync_stroke_color();
            self.lines(radius, screen)
        } else {
            Vec::new()
        }
    }

    fn lines(&self, radius: f32, screen: ScreenParams) -> Vec<Line3D> {
        let mut stroke = self.stroke;
        stroke.width = screen.value_cm_to_px(self.stroke.width);

        let mut points: Vec<Point3D> = Vec::new();

        let mut uv_points: Vec<Point2D> = Vec::new();
        for point2d in &self.points {
            let mut uv = point2d.to_uv(&screen);
            uv.scale(self.scale_factor);
            uv.offset(self.delta_u, self.delta_v);
            uv_points.push(uv);
        }

        let pivot = if self.angle != 0.0 {
            pivot::calculate(&uv_points)
        } else {
            Point2D::new(0.0, 0.0)
        };
        for uv in &mut uv_points {
            if self.angle != 0.0 {
                uv.rotate(self.angle, pivot);
            }

            let point3d = Surface::point(radius, uv.x, uv.y);
            points.push(point3d);
        }

        let mut lines: Vec<Line3D> = Vec::new();
        points.windows(2).for_each(|pair| {
            let line = Line3D::new(pair[0], pair[1], stroke);
            lines.push(line);
        });

        lines
    }

    pub fn apply_parameters(&mut self) {
        self.delta_u += self.display_delta_u;
        self.delta_v += self.display_delta_v;
        self.angle += self.display_angle;
    }

    pub fn default_position(&mut self) {
        self.angle = 0.0;
        self.delta_u = 0.0;
        self.delta_v = 0.0;

        self.display_angle = 0.0;
        self.display_delta_u = 0.0;
        self.display_delta_v = 0.0;
        self.scale_factor = 50.0;
    }

    pub fn default_parameters(&mut self) {
        self.scale_factor = 50.0;
        self.color_contour = colors::PINK;
    }

    pub fn load_texture(&mut self, path: PathBuf) -> Result<(), TextureError> {
        self.points = Vec::new();

        let re = Regex::new(r"X:\s*(-?\d+\.\d+),\s*Y:\s*(-?\d+\.\d+)")
            .map_err(|err| TextureError::InvalidRegexPattern(err.to_string()))?;

        let file = File::open(path).map_err(|err| TextureError::CantOpenFile(err.to_string()))?;

        let reader = io::BufReader::new(file);

        for (line_number, line) in reader.lines().enumerate() {
            let line = line.map_err(|err| TextureError::UndefinedLineString(err.to_string()))?;
            match re.captures(&line) {
                None => Err(TextureError::LineDoesNotContainsPattern(format!(
                    "Line: {}",
                    line_number + 1
                )))?,
                Some(captures) => {
                    let x: f32 = captures[1].parse().map_err(|err: ParseFloatError| {
                        TextureError::ParsingToFloatFailed(err.to_string())
                    })?;
                    let y: f32 = captures[2].parse().map_err(|err: ParseFloatError| {
                        TextureError::ParsingToFloatFailed(err.to_string())
                    })?;

                    self.points.push(Point2D::new(x, y));
                },
            }
        }

        if self.points.is_empty() {
            return Err(TextureError::FileNotContainsPattern);
        }

        self.is_loaded = true;
        Ok(())
    }

    pub fn unload_texture(&mut self) {
        self.points = Vec::new();

        self.is_loaded = false;
    }

    pub fn ensure_texture_loaded(&mut self) {
        if !self.is_loaded {
            self.is_enabled = false;
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.is_loaded
    }

    fn sync_stroke_color(&mut self) {
        self.stroke.color = self.color_contour;
    }
}
