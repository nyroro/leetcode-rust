
impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut total_time = 0;
        for i in 0..points.len() - 1 {
            let current_point = &points[i];
            let next_point = &points[i + 1];
            let horizontal_distance = (current_point[0] - next_point[0]).abs();
            let vertical_distance = (current_point[1] - next_point[1]).abs();
            total_time += horizontal_distance.max(vertical_distance);
        }
        total_time

    }
}
