
impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let total = (n * (n + 1)) / 2;
        let mut sum1 = 0;
        let mut sum2 = total;
        let mut val = 1;
        
        while sum2 > 0 {
            sum1 += val;
            
            if sum1 == sum2 {
                return val;
            }
            
            sum2 -= val;
            val += 1;
        }
        
        -1

    }
}
