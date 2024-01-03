
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if divisor == 0 {
            return 0;
        }
        
        let mut dividend = dividend as i64;
        let mut divisor = divisor as i64;
        
        let sign = if (dividend < 0) ^ (divisor < 0) {
            -1

        } else {
            1

        };
        
        dividend = dividend.abs();
        divisor = divisor.abs();
        
        let mut quotient = 0;
        while dividend >= divisor {
            let mut temp = divisor;
            let mut multiple = 1;
            while dividend >= (temp << 1) {
                temp <<= 1;
                multiple <<= 1;
            }
            dividend -= temp;
            quotient += multiple;
        }
        
        let result = sign * quotient;
        if result < i32::min_value() as i64 {
            return i32::min_value();
        } else if result > i32::max_value() as i64 {
            return i32::max_value();
        } else {
            return result as i32;
        }
    }
}
