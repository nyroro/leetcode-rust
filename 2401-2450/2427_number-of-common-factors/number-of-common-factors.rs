
impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        // 计算a和b的所有因数

        let mut factors_a = Vec::new();
        let mut factors_b = Vec::new();
        
        for i in 1..=a {
            if a % i == 0 {
                factors_a.push(i);
            }
        }
        
        for i in 1..=b {
            if b % i == 0 {
                factors_b.push(i);
            }
        }
        
        // 计算公共因数的数量

        let mut count = 0;
        for factor in factors_a {
            if factors_b.contains(&factor) {
                count += 1;
            }
        }
        
        count

    }
}
