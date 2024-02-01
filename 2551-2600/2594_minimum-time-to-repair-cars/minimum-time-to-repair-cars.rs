
impl Solution {
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        // 创建一个函数来计算每个机械师修理车辆所需的时间

        fn repair_time(rank: i32, cars: i32) -> i64 {
            (rank as i64) * (cars as i64) * (cars as i64)
        }
        
        let mut left = 1;
        let mut right = 1_000_000_000;
        
        while left < right {
            let mid = left + (right - left) / 2;
            let total_cars: i64 = ranks.iter().map(|&r| mid / r as i64).sum();
            
            if total_cars < cars as i64 {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        
        left

    }
}
