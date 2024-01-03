
impl Solution {
    pub fn minimum_perimeter(needed_apples: i64) -> i64 {
        let mut left = 1;
        let mut right = 1_000_000;
        
        while left < right {
            let mid = (left + right) / 2;
            
            let total_apples = 2 * mid * (mid + 1) * (2 * mid + 1);
            
            if total_apples < needed_apples {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        
        return 8 * left;
    }
}
