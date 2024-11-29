use crate::errors::loader::FractalLoaderError;
use crate::model::view::FractalViewModel;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::num::ParseFloatError;

#[derive(Default)]
pub struct FractalLoader {}

impl FractalLoader {
    pub fn load(&self, view_model: &mut FractalViewModel) -> Result<(), FractalLoaderError> {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            let file = File::open(path)
                .map_err(|err| FractalLoaderError::FailedToOpenFile(err.to_string()))?;

            *view_model = Default::default();
            view_model.rules = Vec::with_capacity(3);

            let reader = io::BufReader::new(file);
            let mut lines = Vec::<String>::new();
            for line in reader.lines() {
                let line =
                    line.map_err(|err| FractalLoaderError::FailedToParseLine(err.to_string()))?;
                lines.push(line);
            }

            if lines.len() < 4 {
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

                if !(0.0..=360.0).contains(&angle_degrees) {
                    return Err(FractalLoaderError::WrongAngleValue);
                }

                view_model.angle = angle_degrees;
            } else {
                return Err(FractalLoaderError::AngleNotFound);
            }

            if let Some(iterations) = lines[2].strip_prefix("Iterations = ") {
                let iterations = iterations
                    .parse::<usize>()
                    .map_err(|err| FractalLoaderError::FailedToParseIterations(err.to_string()))?;

                if iterations < 1 {
                    return Err(FractalLoaderError::WrongIterationsValue);
                }

                view_model.iterations = iterations;
            } else {
                return Err(FractalLoaderError::IterationsNotFound);
            }

            for (index, line) in lines[3..].iter().enumerate() {
                if line.len() < 5 || !line[1..=4].eq(" -> ") {
                    return Err(FractalLoaderError::WrongRuleSyntax(format!(
                        "Rule: {}",
                        index + 1
                    )));
                }

                view_model.rules.push(line.clone())
            }
        }

        Ok(())
    }
}
