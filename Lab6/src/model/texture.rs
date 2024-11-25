use crate::errors::texture::TextureError;
use crate::geometry::point2d::Point2D;
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::num::ParseFloatError;
use std::path::PathBuf;

pub struct Texture {
    pub is_enabled: bool,
    is_loaded: bool,

    points: Vec<Point2D>,
}

impl Default for Texture {
    fn default() -> Self {
        Self {
            is_enabled: false,
            is_loaded: false,

            points: Vec::new(),
        }
    }
}

impl Texture {
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
