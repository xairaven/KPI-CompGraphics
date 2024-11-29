use crate::graphics::grid::Grid;
use crate::model::fractal::Fractal;

#[derive(Default)]
pub struct Context {
    pub grid: Grid,
    pub fractal: Fractal,
}
