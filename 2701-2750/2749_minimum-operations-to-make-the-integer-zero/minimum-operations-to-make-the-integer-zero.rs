
impl Solution {
    pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
        for k in 0..=60 {
            let target = (num1 as i64) - (k as i64) * (num2 as i64);
            if target >= 0 && target.count_ones() as i32 <= k && k as i64 <= target {
                return k;
            }
        }
        -1

    }
}
