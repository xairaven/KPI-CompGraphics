use crate::graphics::grid::Grid;
use crate::models::model::Model;
use crate::operations::animation::AnimationSettings;

#[derive(Default)]
pub struct Context {
    pub animation_settings: AnimationSettings,
    pub grid: Grid,
    pub model: Model,
}
