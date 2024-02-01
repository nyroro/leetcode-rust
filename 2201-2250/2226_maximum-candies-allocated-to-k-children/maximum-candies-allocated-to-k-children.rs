
impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let mut left = 1;
        let mut right = *candies.iter().max().unwrap();
        let mut result = 0;
        
        while left <= right {
            let mid = left + (right - left) / 2;
            if Self::can_allocate(&candies, k, mid) {
                result = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        
        result

    }
    
    fn can_allocate(candies: &Vec<i32>, k: i64, max_candies: i32) -> bool {
        let mut count = 0i64;
        for &candy in candies {
            count += i64::from(candy / max_candies);
            if count >= k {
                return true;
            }
        }
        false

    }
}
