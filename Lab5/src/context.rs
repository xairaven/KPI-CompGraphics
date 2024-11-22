use crate::graphics::axes::Axes;
use crate::graphics::model::Model;
use crate::projections::orthographic::OrthographicProjection;
use crate::projections::trimetric::TrimetricProjection;

#[derive(Default)]
pub struct Context {
    pub axes: Axes,
    pub model: Model,
    pub trimetric: TrimetricProjection,
    pub orthographic: OrthographicProjection,
}
