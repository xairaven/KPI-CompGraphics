use crate::graphics::grid::Grid;
use crate::models::model::Model;

#[derive(Default)]
pub struct Context {
    pub grid: Grid,
    pub model: Model,
}
