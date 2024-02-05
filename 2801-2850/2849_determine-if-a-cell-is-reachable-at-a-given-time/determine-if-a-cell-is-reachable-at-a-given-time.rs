
impl Solution {
    pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
        let x_dist = (sx - fx).abs();
        let y_dist = (sy - fy).abs();

        if x_dist == 0 && y_dist == 0 {
            return t != 1;
        }

        x_dist <= t && y_dist <= t

    }
}
