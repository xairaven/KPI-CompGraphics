use crate::errors::texture::TextureError;
use crate::geometry::line3d::Line3D;
use crate::geometry::point2d::Point2D;
use crate::geometry::point3d::Point3D;
use crate::graphics::screen::ScreenParams;
use crate::model::surface::Surface;
use crate::ui::styles::strokes;
use egui::Stroke;
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

            scale_factor: 5.0,

            delta_u: 0.0,
            delta_v: 0.0,
            angle: 0.0,

            points: vec![],

            stroke: strokes::texture_pink(0.1),
        }
    }
}

impl Texture {
    pub fn generate(&mut self, radius: f32, screen: ScreenParams) -> Vec<Line3D> {
        if self.is_enabled {
            self.lines(radius, screen)
        } else {
            Vec::new()
        }
    }

    fn lines(&self, radius: f32, screen: ScreenParams) -> Vec<Line3D> {
        let mut stroke = self.stroke;
        stroke.width = screen.value_cm_to_px(self.stroke.width);

        let mut points: Vec<Point3D> = Vec::new();

        for point2d in &self.points {
            let (u, v) = point2d.to_uv();
            let (u, v) = (u * self.scale_factor, v * self.scale_factor);
            let point3d = Surface::point(radius, u, v);
            points.push(point3d);
        }

        let mut lines: Vec<Line3D> = Vec::new();
        points.windows(2).for_each(|pair| {
            let line = Line3D::new(pair[0], pair[1], stroke);
            lines.push(line);
        });

        lines
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
}
