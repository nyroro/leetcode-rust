
impl Solution {
    pub fn check_overlap(radius: i32, x_center: i32, y_center: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
        // 计算矩形的边界

        let left = x1 as f64;
        let right = x2 as f64;
        let bottom = y1 as f64;
        let top = y2 as f64;
        
        // 计算圆心到矩形的最近点的距离

        let closest_x = if x_center < left as i32 {
            left

        } else if x_center > right as i32 {
            right

        } else {
            x_center as f64

        };
        
        let closest_y = if y_center < bottom as i32 {
            bottom

        } else if y_center > top as i32 {
            top

        } else {
            y_center as f64

        };
        
        let distance = ((x_center as f64 - closest_x).powi(2) + (y_center as f64 - closest_y).powi(2)).sqrt();
        
        // 判断圆和矩形是否重叠

        distance <= radius as f64

    }
}
