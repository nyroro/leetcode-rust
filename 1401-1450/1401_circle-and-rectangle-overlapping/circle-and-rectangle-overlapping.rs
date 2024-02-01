
impl Solution {
    pub fn check_overlap(radius: i32, x_center: i32, y_center: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
        // 计算矩形的边界

        let left = x1;
        let right = x2;
        let bottom = y1;
        let top = y2;
        
        // 计算圆心到矩形的最近点的距离

        let closest_x = if x_center < left {
            left

        } else if x_center > right {
            right

        } else {
            x_center

        };
        
        let closest_y = if y_center < bottom {
            bottom

        } else if y_center > top {
            top

        } else {
            y_center

        };
        
        let distance = ((x_center - closest_x).pow(2) + (y_center - closest_y).pow(2)).sqrt();
        
        // 判断圆和矩形是否重叠

        distance <= radius as f64

    }
}
