use std::path::PathBuf;
use strum_macros::Display;

#[derive(Copy, Clone, Display)]
pub enum FractalExamples {
    #[strum(serialize = "Barnsley's Fern")]
    BarnsleyFern,
}

impl FractalExamples {
    pub fn path(&self) -> PathBuf {
        match self {
            FractalExamples::BarnsleyFern => PathBuf::from(r"examples/Barnsleys-Fern.csv"),
        }
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        [FractalExamples::BarnsleyFern].into_iter()
    }
}
