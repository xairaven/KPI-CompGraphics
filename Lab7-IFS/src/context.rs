use crate::fractal::view::FractalView;
use crate::graphics::grid::Grid;

#[derive(Default)]
pub struct Context {
    pub grid: Grid,

    pub fractal_view: FractalView,
}
