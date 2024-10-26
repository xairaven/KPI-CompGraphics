use crate::graphics::grid::Grid;
use crate::models::model::Model;
use crate::operations::animation::AnimationSettings;
use crate::operations::offset::Offset;
use crate::operations::rotation::Rotation;

#[derive(Default)]
pub struct Context {
    pub animation_settings: AnimationSettings,
    pub grid: Grid,
    pub model: Model,
    pub euclidean_offset: Offset,
    pub euclidean_rotation: Rotation,
}
