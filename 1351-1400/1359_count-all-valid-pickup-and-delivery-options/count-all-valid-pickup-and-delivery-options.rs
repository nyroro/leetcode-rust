
impl Solution {
    pub fn count_orders(n: i32) -> i32 {
        let modulo = 1000000007;
        let mut result: i64 = 1;
        
        for i in 2..=n {
            result = (result * (2*i as i64 - 1) * i as i64) % modulo as i64;
        }
        
        result as i32

    }
}
