use crate::graphics::grid::Grid;
use crate::model::loader::FractalLoader;
use crate::model::view::FractalViewModel;

#[derive(Default)]
pub struct Context {
    pub grid: Grid,

    pub loader: FractalLoader,
    pub fractal_view_model: FractalViewModel,
}
