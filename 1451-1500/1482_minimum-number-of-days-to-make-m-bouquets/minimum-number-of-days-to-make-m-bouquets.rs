
impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        let mut left = 1;
        let mut right = 1_000_000_000;
        let mut result = -1;
        
        while left <= right {
            let mid = left + (right - left) / 2;
            if Self::can_make_bouquets(&bloom_day, m, k, mid) {
                result = mid;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        
        result

    }
    
    fn can_make_bouquets(bloom_day: &Vec<i32>, m: i32, k: i32, wait_days: i32) -> bool {
        let mut flowers = 0;
        let mut bouquets = 0;
        
        for &day in bloom_day.iter() {
            if day > wait_days {
                flowers = 0;
            } else {
                flowers += 1;
                if flowers == k {
                    bouquets += 1;
                    flowers = 0;
                }
            }
        }
        
        bouquets >= m

    }
}
