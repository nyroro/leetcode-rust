


impl Solution {
    pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
        let mut left = 1;
        let mut right = 1_000_000_000;
        while left < right {
            let mid = (left + right) / 2;
            if Self::can_arrive_on_time(&dist, hour, mid as f64) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        if left == 1_000_000_000 {
            -1

        } else {
            left

        }
    }
    
    fn can_arrive_on_time(dist: &Vec<i32>, hour: f64, speed: f64) -> bool {
        let mut time = 0.0;
        for i in 0..dist.len() - 1 {
            time += (dist[i] as f64 / speed).ceil();
        }
        time += dist[dist.len() - 1] as f64 / speed;
        time <= hour

    }
}
