
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
        
        (low - 1) as i32

    }
    
    fn is_possible(n: i64, index: i64, mid: i64, max_sum: i64) -> bool {
        let left = index + 1;
        let right = n - index;
        let mut sum = 0;
        
        if mid > index + 1 {
            sum += (mid + index - left + 1) * (mid - index) / 2;
        } else {
            sum += (mid + 1) * mid / 2 + index - mid + 1;
        }
        
        if mid > n - index {
            sum += (mid + n - index - right + 1) * (mid - (n - index)) / 2;
        } else {
            sum += (mid + 1) * mid / 2 + n - index - mid + 1;
        }
        
        sum - mid <= max_sum

    }
}
