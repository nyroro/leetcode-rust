
impl Solution {
    pub fn get_min_dist_sum(positions: Vec<Vec<i32>>) -> f64 {
        // 计算欧几里得距离

        fn euclidean_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
            ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
        }

        // 定义一个函数来计算服务中心的位置，使得到所有客户的欧几里得距离之和最小

        fn find_min_distance_sum(center_x: f64, center_y: f64, positions: &Vec<Vec<i32>>) -> f64 {
            let mut sum_distance = 0.0;
            for position in positions {
                let x = position[0] as f64;
                let y = position[1] as f64;
                sum_distance += euclidean_distance(center_x, center_y, x, y);
            }
            sum_distance

        }

        // 使用优化算法来找到最小距离和对应的服务中心位置

        let mut center_x = 50.0;  // 初始化服务中心的x坐标

        let mut center_y = 50.0;  // 初始化服务中心的y坐标

        let mut step = 50.0;  // 设置步长

        let mut min_distance_sum = find_min_distance_sum(center_x, center_y, &positions);

        while step > 1e-6 {
            let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];  // 定义四个方向

            let mut found_better = false;
            for dir in &directions {
                let new_x = center_x + step * dir.0 as f64;
                let new_y = center_y + step * dir.1 as f64;
                let new_distance_sum = find_min_distance_sum(new_x, new_y, &positions);
                if new_distance_sum < min_distance_sum {
                    min_distance_sum = new_distance_sum;
                    center_x = new_x;
                    center_y = new_y;
                    found_better = true;
                }
            }
            if !found_better {
                step /= 2.0;
            }
        }

        min_distance_sum

    }
}
