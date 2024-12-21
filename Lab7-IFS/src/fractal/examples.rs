use std::path::PathBuf;
use strum_macros::Display;

#[derive(Copy, Clone, Display)]
pub enum FractalExamples {
    #[strum(serialize = "Barnsley's Fern")]
    BarnsleyFern,

    #[strum(serialize = "Coral")]
    Coral,
}

impl FractalExamples {
    pub fn path(&self) -> PathBuf {
        match self {
            FractalExamples::BarnsleyFern => PathBuf::from(r"examples/Barnsleys-Fern.csv"),
            FractalExamples::Coral => PathBuf::from(r"examples/Coral.csv"),
        }
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        [FractalExamples::BarnsleyFern, FractalExamples::Coral].into_iter()
    }
}
