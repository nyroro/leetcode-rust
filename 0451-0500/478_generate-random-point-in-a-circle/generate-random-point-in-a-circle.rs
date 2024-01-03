
use rand::Rng;

struct Solution {
    radius: f64,
    x_center: f64,
    y_center: f64,
}

impl Solution {
    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Solution {
            radius,
            x_center,
            y_center,
        }
    }

    fn rand_point(&self) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        let r = self.radius * rng.gen::<f64>().sqrt();
        let theta = rng.gen::<f64>() * 2.0 * std::f64::consts::PI;
        let x = self.x_center + r * theta.cos();
        let y = self.y_center + r * theta.sin();
        vec![x, y]
    }
}
