use crate::models::grid::Grid;
use crate::models::model::Model;
use crate::operations::animation::AnimationSettings;
use crate::operations::curve_point::CurvePoint;
use crate::operations::curve_props::CurveProperties;
use crate::operations::offset::Offset;
use crate::operations::rotate::Rotation;

#[derive(Default)]
pub struct Context {
    pub animation_settings: AnimationSettings,
    pub curve_point: CurvePoint,
    pub curve_props: CurveProperties,
    pub grid: Grid,
    pub model: Model,
    pub offset: Offset,
    pub rotation: Rotation,
}
