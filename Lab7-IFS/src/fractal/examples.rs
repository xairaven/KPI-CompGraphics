use std::path::PathBuf;
use strum_macros::Display;

#[derive(Copy, Clone, Display)]
pub enum FractalExamples {
    #[strum(serialize = "Barnsley's Fern")]
    BarnsleyFern,

    #[strum(serialize = "Binary")]
    Binary,

    #[strum(serialize = "Coral")]
    Coral,

    #[strum(serialize = "Crystal")]
    Crystal,

    #[strum(serialize = "Dragon")]
    Dragon,

    #[strum(serialize = "Floor")]
    Floor,

    #[strum(serialize = "Koch-3")]
    Koch3,

    #[strum(serialize = "Spiral")]
    Spiral,

    #[strum(serialize = "Tree")]
    Tree,

    #[strum(serialize = "Triangle")]
    Triangle,

    #[strum(serialize = "Whirlpool")]
    Whirlpool,

    #[strum(serialize = "Zigzag")]
    Zigzag,
}

impl FractalExamples {
    pub fn path(&self) -> PathBuf {
        match self {
            FractalExamples::BarnsleyFern => PathBuf::from(r"examples/Barnsleys-Fern.csv"),
            FractalExamples::Binary => PathBuf::from(r"examples/Binary.csv"),
            FractalExamples::Coral => PathBuf::from(r"examples/Coral.csv"),
            FractalExamples::Crystal => PathBuf::from(r"examples/Crystal.csv"),
            FractalExamples::Dragon => PathBuf::from(r"examples/Dragon.csv"),
            FractalExamples::Floor => PathBuf::from(r"examples/Floor.csv"),
            FractalExamples::Koch3 => PathBuf::from(r"examples/Koch-3.csv"),
            FractalExamples::Spiral => PathBuf::from(r"examples/Spiral.csv"),
            FractalExamples::Tree => PathBuf::from(r"examples/Tree.csv"),
            FractalExamples::Triangle => PathBuf::from(r"examples/Triangle.csv"),
            FractalExamples::Whirlpool => PathBuf::from(r"examples/Whirlpool.csv"),
            FractalExamples::Zigzag => PathBuf::from(r"examples/Zigzag.csv"),
        }
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        [
            FractalExamples::BarnsleyFern,
            FractalExamples::Binary,
            FractalExamples::Coral,
            FractalExamples::Crystal,
            FractalExamples::Dragon,
            FractalExamples::Floor,
            FractalExamples::Koch3,
            FractalExamples::Spiral,
            FractalExamples::Tree,
            FractalExamples::Triangle,
            FractalExamples::Whirlpool,
            FractalExamples::Zigzag,
        ]
        .into_iter()
    }
}
