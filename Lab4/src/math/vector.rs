use crate::traits::positionable::Positionable;

pub fn length<T: Positionable>(point: T) -> f32 {
    f32::sqrt(point.x().powf(2.0) + point.y().powf(2.0))
}
