
impl Solution {
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        let modulo = 1_000_000_007;
        let min_val = a.min(b) as i64;
        let max_val = a.max(b) as i64;
        let lcm = Solution::lcm(a, b) as i64;

        let mut left = 0;
        let mut right = n as i64 * min_val * 2;

        while left < right {
            let mid = left + (right - left) / 2;
            let count = mid / min_val + mid / max_val - mid / lcm;

            if count < n as i64 {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        (left % modulo) as i32

    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a

    }

    fn lcm(a: i32, b: i32) -> i32 {
        a * b / Solution::gcd(a, b)
    }
}
