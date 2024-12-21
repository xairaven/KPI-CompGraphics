use crate::fractal::file_loader::FileLoader;
use crate::fractal::state::FractalState;
use crate::graphics::grid::Grid;

#[derive(Default)]
pub struct Context {
    pub grid: Grid,

    pub fractal_state: FractalState,
    pub file_loader: FileLoader,
}
