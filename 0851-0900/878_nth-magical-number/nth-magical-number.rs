
impl Solution {
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        let modulo = 1000000007;
        let min_val = a.min(b);
        let max_val = a.max(b);
        let mut left = 0;
        let mut right = (n as i64 * min_val as i64) as i32;

        while left < right {
            let mid = left + (right - left) / 2;
            let count = mid / a + mid / b - mid / (a * b / gcd(a, b));

            if count < n {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left % modulo

    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a

        } else {
            Solution::gcd(b, a % b)
        }
    }
}
