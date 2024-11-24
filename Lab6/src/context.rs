use crate::graphics::axes::Axes;
use crate::model::surface::Surface;
use crate::model::texture::Texture;
use crate::operations::animation::Animation;
use crate::projections::orthographic::OrthographicProjection;
use crate::projections::trimetric::TrimetricProjection;
use crate::transformations::offset::Offset;
use crate::transformations::rotation::Rotation;

#[derive(Default)]
pub struct Context {
    pub axes: Axes,

    pub surface: Surface,
    pub texture: Texture,

    pub trimetric: TrimetricProjection,
    pub orthographic: OrthographicProjection,

    pub animation: Animation,

    pub offset: Offset,
    pub rotation: Rotation,
}
