use crate::geometry::line::Line;
use crate::geometry::point::Point;
use crate::graphics::screen::ScreenParams;
use crate::models::bezier_curve::bezier_curve;
use crate::models::bezier_point::BezierPoint;
use crate::traits::positionable::Positionable;
use crate::ui::styles::{colors, strokes};
use egui::{Color32, Stroke};

pub struct Model {
    pub points: Vec<BezierPoint>,

    pub outline: Stroke,
    pub fill_control: Color32,
    pub fill_defining: Color32,

    pub skeleton_stroke: Stroke,

    pub model_stroke: Stroke,
    pub bezier_step: f32,

    pub are_tooltips_enabled: bool,
    pub is_skeleton_enabled: bool,

    pub radius: f32,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            points: Self::default_points(),
            outline: strokes::bezier_outline(0.02),
            fill_control: colors::RED,
            fill_defining: colors::GREEN,

            skeleton_stroke: strokes::skeleton_dark_grey(0.05),

            model_stroke: strokes::model_black(0.1),
            bezier_step: 0.1,

            are_tooltips_enabled: false,
            is_skeleton_enabled: false,

            radius: 0.1,
        }
    }
}

impl Model {
    pub fn skeleton_lines(&self, screen_params: ScreenParams) -> Vec<Line<Point>> {
        let mut stroke = self.skeleton_stroke;
        stroke.width = screen_params.value_cm_to_px(self.skeleton_stroke.width);

        self.points
            .windows(2)
            .map(|pair| {
                let start = Point::new(pair[0].point.x, pair[0].point.y);
                let end = Point::new(pair[1].point.x, pair[1].point.y);

                Line::new(start, end, stroke)
            })
            .collect()
    }

    pub fn lines(&self, screen_params: ScreenParams) -> Vec<Line<Point>> {
        let mut stroke = self.model_stroke;
        stroke.width = screen_params.value_cm_to_px(self.model_stroke.width);

        assert_eq!((self.points.len() - 1) % 2, 0);

        let mut vec: Vec<Line<Point>> = vec![];

        let step = 2;
        let chunk_size = 3;

        for i in (0..self.points.len().saturating_sub(chunk_size - 1)).step_by(step) {
            let control_first = &self.points[i];
            let defining = &self.points[i + 1];
            let control_second = &self.points[i + 2];

            bezier_curve(
                &mut vec,
                &stroke,
                self.bezier_step,
                &control_first.point,
                &control_second.point,
                &defining.point,
            )
        }

        vec
    }

    pub fn default_points() -> Vec<BezierPoint> {
        vec![
            BezierPoint::control(-4.959, -0.960),           // 1
            BezierPoint::defining(-4.699, -0.224),          // 2
            BezierPoint::control(-4.424, -0.512),           // 3
            BezierPoint::defining(-0.960, 5.759),           // 4
            BezierPoint::control(-0.279, 8.959),            // 5
            BezierPoint::defining(-0.430, 7.099),           // 6
            BezierPoint::control(0.024, 5.749),             // 7
            BezierPoint::defining(6.520, 2.682),            // 8
            BezierPoint::control(1.445, -2.384),            // 9
            BezierPoint::defining(0.672, -2.046),           // 10
            BezierPoint::control(-0.009, -1.935),           // 11
            BezierPoint::defining(-0.115, -2.010),          // 12
            BezierPoint::control(-0.259, -2.060),           // 13
            BezierPoint::defining(-0.780, -1.880),          // 14
            BezierPoint::control(-1.670, -1.819),           // 15
            BezierPoint::defining(-0.803, -1.330),          // 16
            BezierPoint::control(-0.038, -1.059),           // 17
            BezierPoint::defining(1.036, 0.476),            // 18
            BezierPoint::control(1.814, 1.818),             // 19
            BezierPoint::defining(1.914, 2.628),            // 20
            BezierPoint::control(0.024, 5.749),             // 21
            BezierPoint::defining(-0.215, 5.819),           // 22
            BezierPoint::control(-0.525, 5.903),            // 23
            BezierPoint::defining(-0.215, 5.819),           // 24
            BezierPoint::control(0.024, 5.749),             // 25
            BezierPoint::defining(-0.244, 6.612),           // 26
            BezierPoint::control(-0.333, 7.744),            // 27
            BezierPoint::defining(-0.820, 2.840),           // 28
            BezierPoint::control(-1.989, -3.423),           // 29
            BezierPoint::defining(-1.310, -3.380),          // 30
            BezierPoint::control(-0.749, -3.514),           // 31
            BezierPoint::defining(-0.579, -0.459),          // 32
            BezierPoint::control(-0.525, 5.903),            // 33
            BezierPoint::defining(-0.551, 6.889),           // 34
            BezierPoint::control(-0.426, 7.734),            // 35
            BezierPoint::defining(-0.751, 4.281),           // 36
            BezierPoint::control(-1.753, -1.760),           // 37
            BezierPoint::defining(-1.198, -1.435),          // 38
            BezierPoint::control(-0.636, -1.307),           // 39
            BezierPoint::defining(-0.090, -1.386),          // 40
            BezierPoint::control(0.363, -1.433),            // 41
            BezierPoint::defining(-0.076, -1.775),          // 42
            BezierPoint::control(-0.494, -1.958),           // 43
            BezierPoint::defining(-1.129, -1.734),          // 44
            BezierPoint::control(-1.780, -1.760),           // 45
            BezierPoint::defining(-1.986, -2.855),          // 46
            BezierPoint::control(-2.094, -3.470),           // 47
            BezierPoint::defining(-5.531, -3.338),          // 48
            BezierPoint::control(-5.651, -3.403),           // 49
            BezierPoint::defining(-6.178, -3.475),          // 50
            BezierPoint::control(-6.274, -3.520),           // 51
            BezierPoint::defining(-5.647, -4.528),          // 52
            BezierPoint::control(-5.102, -4.713),           // 53
            BezierPoint::defining(-5.476, -4.866),          // 54
            BezierPoint::control(-5.865, -4.850),           // 55
            BezierPoint::defining(-5.246, -4.949),          // 56
            BezierPoint::control(-4.227, -4.557),           // 57
            BezierPoint::defining(-4.009, -4.825),          // 58
            BezierPoint::control(-3.578, -4.784),           // 59
            BezierPoint::defining(-3.426, -4.890),          // 60
            BezierPoint::control(-3.223, -4.980),           // 61
            BezierPoint::defining(-3.380, -4.736),          // 62
            BezierPoint::control(-3.589, -4.653),           // 63
            BezierPoint::defining(-3.180, -4.768),          // 64
            BezierPoint::control(-2.969, -4.893),           // 65
            BezierPoint::defining(-3.056, -4.699),          // 66
            BezierPoint::control(-3.275, -4.622),           // 67
            BezierPoint::defining(-2.595, -4.756),          // 68
            BezierPoint::control(-2.326, -4.980),           // 69
            BezierPoint::defining(-2.385, -4.750),          // 70
            BezierPoint::control(-2.580, -4.617),           // 71
            BezierPoint::defining(-2.282, -4.665),          // 72
            BezierPoint::control(-2.077, -4.898),           // 73
            BezierPoint::defining(-2.067, -4.594),          // 74
            BezierPoint::control(-2.352, -4.503),           // 75
            BezierPoint::defining(-2.027, -4.394),          // 76
            BezierPoint::control(-1.708, -4.508),           // 77
            BezierPoint::defining(-1.111, -4.672),          // 78
            BezierPoint::control(-0.816, -4.791),           // 79
            BezierPoint::defining(-0.566, -4.783),          // 80
            BezierPoint::control(-0.383, -4.958),           // 81
            BezierPoint::defining(-0.459, -4.642),          // 82
            BezierPoint::control(-0.745, -4.557),           // 83
            BezierPoint::defining(-0.330, -4.574),          // 84
            BezierPoint::control(-0.064, -4.857),           // 85
            BezierPoint::defining(-0.146, -4.475),          // 86
            BezierPoint::control(-0.320, -4.378),           // 87
            BezierPoint::defining(0.078, -4.751),           // 88
            BezierPoint::control(0.408, -4.857),            // 89
            BezierPoint::defining(0.694, -4.882),           // 90
            BezierPoint::control(0.870, -4.897),            // 91
            BezierPoint::defining(1.121, -4.853),           // 92
            BezierPoint::control(1.391, -4.984),            // 93
            BezierPoint::defining(1.235, -4.725),           // 94
            BezierPoint::control(0.939, -4.697),            // 95
            BezierPoint::defining(1.475, -4.716),           // 96
            BezierPoint::control(1.862, -4.961),            // 97
            BezierPoint::defining(1.713, -4.649),           // 98
            BezierPoint::control(1.359, -4.551),            // 99
        ]
    }
}
