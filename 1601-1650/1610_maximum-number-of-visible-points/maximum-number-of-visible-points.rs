
impl Solution {
    pub fn visible_points(points: Vec<Vec<i32>>, angle: i32, location: Vec<i32>) -> i32 {
        use std::f64::consts::PI;

        let (x, y) = (location[0] as f64, location[1] as f64);
        let mut angles = Vec::new();
        let mut same = 0;

        for point in &points {
            let (dx, dy) = (point[0] as f64 - x, point[1] as f64 - y);
            if dx == 0.0 && dy == 0.0 {
                same += 1;
            } else {
                angles.push((dy.atan2(dx) * 180.0 / PI).rem_euclid(360.0));
            }
        }

        angles.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut max_count = 0;
        let n = angles.len();
        for i in 0..n {
            angles.push(angles[i] + 360.0);
        }

        let mut j = 0;
        for i in 0..n {
            while j < 2 * n && angles[j] - angles[i] <= angle as f64 {
                j += 1;
            }
            max_count = max_count.max(j - i);
        }

        max_count as i32 + same

    }
}
