
impl Solution {
    pub fn num_points(darts: Vec<Vec<i32>>, r: i32) -> i32 {
        let mut ans = 1;
        for point in &darts {
            let mut angles = Vec::new();
            for other_point in &darts {
                if point != other_point {
                    let d = (((other_point[0] - point[0]).pow(2) + (other_point[1] - point[1]).pow(2)) as f64).sqrt();
                    if d <= 2.0 * r as f64 {
                        let angle = ((other_point[1] - point[1]) as f64).atan2((other_point[0] - point[0]) as f64);
                        let delta = (d / (2.0 * r as f64)).acos();
                        angles.push((angle - delta, 1)); // entry

                        angles.push((angle + delta, -1)); // exit

                    }
                }
            }
            angles.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap().then_with(|| b.1.cmp(&a.1)));
            let mut val = 1;
            for (_, entry) in angles {
                ans = ans.max(val + entry);
                val += entry;
            }
        }
        ans

    }
}
