use crate::graphics::grid::Grid;
use crate::models::animation::AnimationSettings;
use crate::models::model::Model;

#[derive(Default)]
pub struct Context {
    pub animation_settings: AnimationSettings,
    pub grid: Grid,
    pub model: Model,
}
