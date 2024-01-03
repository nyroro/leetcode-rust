
impl Solution {
    pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
        let (n, index, max_sum) = (n as i64, index as i64, max_sum as i64);
        let mut low = 1;
        let mut high = max_sum + 1;
        
        while low < high {
            let mid = low + (high - low) / 2;
            if Self::is_possible(n, index, mid, max_sum) {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        
        low as i32 - 1

    }
    
    fn is_possible(n: i64, index: i64, mid: i64, max_sum: i64) -> bool {
        let left = index + 1;
        let right = n - index;
        
        let sum_left = if mid > left {
            (mid + mid - left + 1) * (left) / 2

        } else {
            (mid + 1) * mid / 2 + left - mid

        };
        
        let sum_right = if mid > right {
            (mid + mid - right + 1) * (right) / 2

        } else {
            (mid + 1) * mid / 2 + right - mid

        };
        
        sum_left + sum_right - mid <= max_sum

    }
}
