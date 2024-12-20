use crate::errors::loader::FractalLoaderError;
use crate::model::view::FractalViewModel;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::num::ParseFloatError;
use std::path::PathBuf;

pub struct FractalLoader {
    fields: usize,
}

impl Default for FractalLoader {
    fn default() -> Self {
        Self { fields: 5 }
    }
}

impl FractalLoader {
    pub fn load_with_file_pick(
        &self, view_model: &mut FractalViewModel,
    ) -> Result<(), FractalLoaderError> {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            self.load_from_path(view_model, path)?
        }

        Ok(())
    }

    pub fn load_from_path(
        &self, view_model: &mut FractalViewModel, path: PathBuf,
    ) -> Result<(), FractalLoaderError> {
        let file = File::open(path)
            .map_err(|err| FractalLoaderError::FailedToOpenFile(err.to_string()))?;

        view_model.reset_with_empty_rules();

        let reader = io::BufReader::new(file);
        let mut lines = Vec::<String>::new();
        for line in reader.lines() {
            let line =
                line.map_err(|err| FractalLoaderError::FailedToParseLine(err.to_string()))?;
            lines.push(line);
        }

        if lines.len() < self.fields {
            return Err(FractalLoaderError::NotEnoughData);
        }

        if let Some(axiom) = lines[0].strip_prefix("Axiom = ") {
            view_model.axiom = String::from(axiom);
        } else {
            return Err(FractalLoaderError::AxiomNotFound);
        }

        if let Some(angle) = lines[1].strip_prefix("Angle = ") {
            let angle_degrees = angle.parse::<f32>().map_err(|err: ParseFloatError| {
                FractalLoaderError::FailedToParseAngle(err.to_string())
            })?;

            view_model.angle = angle_degrees;
        } else {
            return Err(FractalLoaderError::AngleNotFound);
        }

        if let Some(angle) = lines[2].strip_prefix("Initial Angle = ") {
            let initial_angle_degrees = angle.parse::<f32>().map_err(|err: ParseFloatError| {
                FractalLoaderError::FailedToParseInitialAngle(err.to_string())
            })?;

            view_model.initial_angle = initial_angle_degrees;
        } else {
            return Err(FractalLoaderError::InitialAngleNotFound);
        }

        if let Some(iterations) = lines[3].strip_prefix("Iterations = ") {
            let iterations = iterations
                .parse::<usize>()
                .map_err(|err| FractalLoaderError::FailedToParseIterations(err.to_string()))?;

            view_model.iterations = iterations;
        } else {
            return Err(FractalLoaderError::IterationsNotFound);
        }

        for line in lines[4..].iter() {
            view_model.rules.push(line.clone())
        }

        Ok(())
    }
}
