use crate::graphics::grid::Grid;
use crate::graphics::model::Model;
use crate::projections::orthographic::OrthographicProjection;
use crate::projections::trimetric::TrimetricProjection;

#[derive(Default)]
pub struct Context {
    pub grid: Grid,
    pub model: Model,
    pub trimetric: TrimetricProjection,
    pub orthographic: OrthographicProjection,
}
