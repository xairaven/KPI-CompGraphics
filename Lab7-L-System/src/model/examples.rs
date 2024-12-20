use std::path::PathBuf;
use strum_macros::Display;

#[derive(Copy, Clone, Display)]
pub enum FractalExamples {
    #[strum(serialize = "Dragon Curve")]
    DragonCurve,

    #[strum(serialize = "Gosper Curve")]
    GosperCurve,

    #[strum(serialize = "Hilbert Curve")]
    HilbertCurve,

    #[strum(serialize = "Koch Curve")]
    KochCurve,

    #[strum(serialize = "Koch Quadratic Curve")]
    KochQuadraticCurve,

    #[strum(serialize = "Koch Quadratic Snowflake")]
    KochQuadraticSnowflake,

    #[strum(serialize = "Koch Snowflake")]
    KochSnowflake,

    #[strum(serialize = "L-System Bush 1")]
    LsystemBush1,

    #[strum(serialize = "L-System Bush 2")]
    LsystemBush2,

    #[strum(serialize = "L-System Bush 3")]
    LsystemBush3,

    #[strum(serialize = "L-System Sticks 1")]
    LsystemSticks1,

    #[strum(serialize = "L-System Sticks 2")]
    LsystemSticks2,

    #[strum(serialize = "Peano Curve")]
    PeanoFractal,

    #[strum(serialize = "Penrose Tiling")]
    PenroseTiling,

    #[strum(serialize = "Sierpiński Curve")]
    SierpinskiCurve,

    #[strum(serialize = "Sierpiński Rhombus")]
    SierpinskiRhombus,

    #[strum(serialize = "Sierpiński Triangle")]
    SierpinskiTriangle,
}

impl FractalExamples {
    pub fn path(&self) -> PathBuf {
        match self {
            FractalExamples::DragonCurve => PathBuf::from(r"examples/DragonCurve.txt"),
            FractalExamples::GosperCurve => PathBuf::from(r"examples/GosperCurve.txt"),
            FractalExamples::HilbertCurve => PathBuf::from(r"examples/HilbertCurve.txt"),
            FractalExamples::KochCurve => PathBuf::from(r"examples/KochCurve.txt"),
            FractalExamples::KochQuadraticCurve => {
                PathBuf::from(r"examples/KochQuadraticCurve.txt")
            },
            FractalExamples::KochQuadraticSnowflake => {
                PathBuf::from(r"examples/KochQuadraticSnowflake.txt")
            },
            FractalExamples::KochSnowflake => PathBuf::from(r"examples/KochSnowflake.txt"),
            FractalExamples::LsystemBush1 => PathBuf::from(r"examples/LsystemBush-1.txt"),
            FractalExamples::LsystemBush2 => PathBuf::from(r"examples/LsystemBush-2.txt"),
            FractalExamples::LsystemBush3 => PathBuf::from(r"examples/LsystemBush-3.txt"),
            FractalExamples::LsystemSticks1 => PathBuf::from(r"examples/LsystemSticks-1.txt"),
            FractalExamples::LsystemSticks2 => PathBuf::from(r"examples/LsystemSticks-2.txt"),
            FractalExamples::PeanoFractal => PathBuf::from(r"examples/PeanoFractal.txt"),
            FractalExamples::PenroseTiling => PathBuf::from(r"examples/PenroseTiling.txt"),
            FractalExamples::SierpinskiCurve => PathBuf::from(r"examples/SierpinskiCurve.txt"),
            FractalExamples::SierpinskiRhombus => PathBuf::from(r"examples/SierpinskiRhombus.txt"),
            FractalExamples::SierpinskiTriangle => {
                PathBuf::from(r"examples/SierpinskiTriangle.txt")
            },
        }
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        [
            FractalExamples::DragonCurve,
            FractalExamples::GosperCurve,
            FractalExamples::HilbertCurve,
            FractalExamples::KochCurve,
            FractalExamples::KochQuadraticCurve,
            FractalExamples::KochQuadraticSnowflake,
            FractalExamples::KochSnowflake,
            FractalExamples::LsystemBush1,
            FractalExamples::LsystemBush2,
            FractalExamples::LsystemBush3,
            FractalExamples::LsystemSticks1,
            FractalExamples::LsystemSticks2,
            FractalExamples::PeanoFractal,
            FractalExamples::PenroseTiling,
            FractalExamples::SierpinskiCurve,
            FractalExamples::SierpinskiRhombus,
            FractalExamples::SierpinskiTriangle,
        ]
        .into_iter()
    }
}
