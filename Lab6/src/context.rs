use crate::graphics::axes::Axes;
use crate::graphics::model::Model;
use crate::operations::animation::Animation;
use crate::projections::orthographic::OrthographicProjection;
use crate::projections::trimetric::TrimetricProjection;
use crate::transformations::offset::Offset;
use crate::transformations::rotation::Rotation;

#[derive(Default)]
pub struct Context {
    pub axes: Axes,

    pub model: Model,

    pub trimetric: TrimetricProjection,
    pub orthographic: OrthographicProjection,

    pub animation: Animation,

    pub offset: Offset,
    pub rotation: Rotation,
}
