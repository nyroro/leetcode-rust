
impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let mut left = 0;
        let mut right = n;
        while left <= right {
            let mid = left + (right - left) / 2;
            let curr = (mid as i64 * (mid as i64 + 1)) / 2;
            if curr == n as i64 {
                return mid;
            } else if curr < n as i64 {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        right as i32

    }
}
