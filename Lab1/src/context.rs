use crate::models::grid::Grid;
use crate::models::model::Model;
use crate::transformations::affine::Affine;
use crate::transformations::euclidean::Euclidean;
use crate::transformations::projective::Projective;
use crate::transformations::resize::Resize;

#[derive(Default)]
pub struct Context {
    pub grid: Grid,
    pub model: Model,

    pub affine: Affine,
    pub euclidean: Euclidean,
    pub projective: Projective,
    pub resize: Resize,
}
