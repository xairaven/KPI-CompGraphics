use crate::geometry::moveable_point::MoveablePoint;
use crate::models::model::Model;
use crate::traits::positionable::Positionable;
use crate::ui::styles::colors;
use egui::Color32;

pub struct Offset {
    pub is_enabled: bool,
    pub dot: MoveablePoint,

    pub old: (f32, f32),

    pub color: Color32,
}

impl Default for Offset {
    fn default() -> Self {
        Self {
            is_enabled: false,
            dot: MoveablePoint::new(0.0, 0.0),
            old: (0.0, 0.0),

            color: colors::BLUE,
        }
    }
}

impl Offset {
    pub fn process(&mut self, model: &mut Model) {
        if self.dot.x == 0.0 && self.dot.y == 0.0 {
            return;
        };

        let delta = (self.dot.x - self.old.0, self.dot.y - self.old.1);
        model.points.iter_mut().for_each(|bezier| {
            self.update_point(&mut bezier.point, delta);
        });
        self.old = (self.dot.x, self.dot.y);
    }

    pub fn update_point(&mut self, point: &mut MoveablePoint, delta: (f32, f32)) {
        point.x += delta.0;
        point.y += delta.1;
    }

    pub fn checkout_status(&mut self, model: &mut Model) {
        let enabled = self.is_enabled;

        self.old = (0.0, 0.0);
        self.dot = MoveablePoint::new(0.0, 0.0);

        if enabled {
            let first = model.points.first();
            if let Some(bezier) = first {
                self.dot.x = bezier.point.x;
                self.dot.y = bezier.point.y;

                self.old = (bezier.point.x, bezier.point.y);
            }
        }
    }
}
