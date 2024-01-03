
impl Solution {
    pub fn preimage_size_fzf(k: i32) -> i32 {
        let mut left = 0;
        let mut right = 5 * (k as i64) + 1;
        while left < right {
            let mid = left + (right - left) / 2;
            let zeros = Self::trailing_zeros(mid);
            if zeros < k {
                left = mid + 1;
            } else if zeros > k {
                right = mid;
            } else {
                return 5;
            }
        }
        0

    }

    fn trailing_zeros(n: i64) -> i32 {
        let mut count = 0;
        let mut d = n / 5;
        while d > 0 {
            count += d;
            d /= 5;
        }
        count as i32

    }
}
