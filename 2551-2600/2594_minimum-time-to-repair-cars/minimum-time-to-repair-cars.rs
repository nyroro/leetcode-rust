
impl Solution {
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let mut left = 0;
        let mut right = 1_000_000_000_000_000;
        
        while left < right {
            let mid = left + (right - left) / 2;
            let total_cars: i64 = ranks.iter().map(|&r| (mid as f64 / r as f64).sqrt() as i64).sum();
            
            if total_cars < cars as i64 {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        
        right

    }
}
