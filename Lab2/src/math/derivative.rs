pub fn derivative(x: f32, y: f32, a: f32, b: f32) -> f32 {
    -1.0 * (4.0 * a.powf(2.0) * x - 6.0 * a * x.powf(2.0) - 2.0 * a * y.powf(2.0) - b.powf(2.0) * x
        + 2.0 * x.powf(3.0)
        + 2.0 * x * y.powf(2.0))
        / (y * (-4.0 * a * x - b.powf(2.0) + 2.0 * x.powf(2.0) + 2.0 * y.powf(2.0)))
}
