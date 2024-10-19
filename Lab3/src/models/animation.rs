use crate::models::animation::Direction::ToFigure;

pub struct AnimationSettings {
    pub is_running: bool,

    pub direction: Direction,
}

impl Default for AnimationSettings {
    fn default() -> Self {
        Self {
            is_running: false,
            direction: ToFigure,
        }
    }
}

impl AnimationSettings {
    // pub fn step(&mut self, model: &mut Model) {
    //     model.a += self.delta * self.direction;
    //     model.b += self.delta * self.direction;
    //
    //     if model.a >= model::PARAMETERS_MAX as f32 / 2.0
    //         || model.b >= model::PARAMETERS_MAX as f32 / 2.0
    //         || model.a <= -model::PARAMETERS_MAX as f32 / 2.0
    //         || model.b <= -model::PARAMETERS_MAX as f32 / 2.0
    //     {
    //         self.direction *= -1.0;
    //     }
    // }
}

pub enum Direction {
    ToModel,
    ToFigure,
}
