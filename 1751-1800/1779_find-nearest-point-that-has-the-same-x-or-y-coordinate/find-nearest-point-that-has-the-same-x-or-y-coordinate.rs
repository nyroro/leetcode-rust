
impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let mut min_distance = i32::MAX;
        let mut min_index = -1;
        
        for (index, point) in points.iter().enumerate() {
            let x_diff = (x - point[0]).abs();
            let y_diff = (y - point[1]).abs();
            
            if x_diff == 0 || y_diff == 0 {
                let distance = x_diff + y_diff;
                if distance < min_distance {
                    min_distance = distance;
                    min_index = index as i32;
                } else if distance == min_distance && index as i32 < min_index {
                    min_index = index as i32;
                }
            }
        }
        
        min_index

    }
}
