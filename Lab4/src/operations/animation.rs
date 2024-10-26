use crate::models::bezier_point::BezierPoint;
use crate::models::model::Model;

pub struct AnimationSettings {
    pub is_running: bool,

    figure_state: Vec<BezierPoint>,
    model_state: Vec<BezierPoint>,

    step: i32,
    max_steps: i32,
    direction: i32,
}

impl Default for AnimationSettings {
    fn default() -> Self {
        Self {
            is_running: false,
            figure_state: Self::figure_state(),
            model_state: vec![],

            step: 0,
            max_steps: 150,
            direction: 1,
        }
    }
}

impl AnimationSettings {
    pub fn process_animation(&mut self, model: &mut Model) {
        assert_eq!(model.points.len(), self.figure_state.len());
        assert_eq!(model.points.len(), self.model_state.len());
        assert_eq!(model.points.len(), 121);

        let length = model.points.len();
        for i in 0..length {
            let x = self.model_state[i].point.x
                * (1.0 - (self.step as f32 / self.max_steps as f32))
                + self.figure_state[i].point.x * (self.step as f32 / self.max_steps as f32);
            let y = self.model_state[i].point.y
                * (1.0 - (self.step as f32 / self.max_steps as f32))
                + self.figure_state[i].point.y * (self.step as f32 / self.max_steps as f32);

            model.points[i].point.x = x;
            model.points[i].point.y = y;
        }
        model.points[120] = model.points[0];

        self.step += self.direction;

        if self.step > self.max_steps {
            self.step = self.max_steps;
            self.direction *= -1;
        }

        if self.step < 0 {
            self.step = 0;
            self.direction *= -1;
        }
    }

    fn figure_state() -> Vec<BezierPoint> {
        let mut points: Vec<BezierPoint> = Vec::with_capacity(121);

        let max_points = 120;
        let angle_step = 2.0 * (std::f64::consts::PI as f32) / max_points as f32;
        let radius = 3.0;

        let mut counter = 0;
        while counter < max_points {
            let angle = counter as f32 * angle_step;
            let (x, y) = (radius * angle.cos(), radius * angle.sin());
            points.push(BezierPoint::control(x, y));

            let angle = (counter + 1) as f32 * angle_step;
            let (x, y) = (radius * angle.cos(), radius * angle.sin());
            points.push(BezierPoint::defining(x, y));

            counter += 2;
        }
        points.push(points[0]);

        points
    }

    pub fn checkout_status(&mut self, model: &mut Model) {
        self.is_running = !self.is_running;

        if self.is_running {
            self.model_state = model.points.clone();
        } else {
            self.model_state = vec![];
        }
    }
}
