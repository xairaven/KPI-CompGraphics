use crate::errors::file_loader::FractalLoaderError;
use crate::fractal::state::FractalState;
use std::fs::File;
use std::path::PathBuf;

#[derive(Default)]
pub struct FileLoader;

impl FileLoader {
    pub fn load_with_file_pick(&self, state: &mut FractalState) -> Result<(), FractalLoaderError> {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            self.load_from_path(state, path)?
        }

        Ok(())
    }

    pub fn load_from_path(
        &self, state: &mut FractalState, path: PathBuf,
    ) -> Result<(), FractalLoaderError> {
        let file = File::open(path)
            .map_err(|err| FractalLoaderError::FailedToOpenFile(err.to_string()))?;

        *state = Default::default();
        state.empty_systems();

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(file);

        for (index, result) in rdr.records().enumerate() {
            let record = result.map_err(|err| {
                FractalLoaderError::FailedToParseRecord(format!("Record #{}: {}", index, err))
            })?;

            if record.len() != 7 {
                return Err(FractalLoaderError::InvalidRecordLength(format!(
                    "Record #{}: expected 7 elements, found {}",
                    index,
                    record.len()
                )));
            }

            let mut system: [f32; 7] = [0.0; 7];
            for (index, item) in record.iter().enumerate() {
                let number = item.parse::<f32>().map_err(|_| {
                    FractalLoaderError::FailedToParseFloat(format!("Element: \"{}\"", item))
                })?;
                system[index] = number;
            }
            state.push_system(system);
        }

        if state.systems.is_empty() {
            return Err(FractalLoaderError::EmptyFile);
        }

        state
            .initialize()
            .map_err(FractalLoaderError::ValidationError)?;

        Ok(())
    }
}
