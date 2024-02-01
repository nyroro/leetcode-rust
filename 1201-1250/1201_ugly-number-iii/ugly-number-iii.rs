
impl Solution {
    pub fn nth_ugly_number(n: i32, a: i32, b: i32, c: i32) -> i32 {
        let mut left = 1;
        let mut right = 2 * 10i64.pow(9);
        let ab = a as i64 * b as i64 / Solution::gcd(a as i64, b as i64);
        let ac = a as i64 * c as i64 / Solution::gcd(a as i64, c as i64);
        let bc = b as i64 * c as i64 / Solution::gcd(b as i64, c as i64);
        let abc = a as i64 * bc / Solution::gcd(a as i64, bc as i64);
        
        while left < right {
            let mid = left + (right - left) / 2;
            let count = mid / a as i64 + mid / b as i64 + mid / c as i64 - mid / ab - mid / ac - mid / bc + mid / abc;
            if count < n as i64 {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        
        left as i32

    }
    
    fn gcd(mut x: i64, mut y: i64) -> i64 {
        while y != 0 {
            let temp = x % y;
            x = y;
            y = temp;
        }
        x

    }
}
