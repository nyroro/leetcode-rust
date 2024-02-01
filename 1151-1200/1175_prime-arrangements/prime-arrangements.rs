
impl Solution {
    pub fn num_prime_arrangements(n: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        
        // 判断一个数是否为素数

        fn is_prime(num: i32) -> bool {
            if num < 2 {
                return false;
            }
            for i in 2..=(num as f64).sqrt() as i32 {
                if num % i == 0 {
                    return false;
                }
            }
            true

        }
        
        let prime_count = (2..=n).filter(|&x| is_prime(x)).count() as i64;
        let non_prime_count = n as i64 - prime_count;
        
        // 计算素数的排列数量

        let mut prime_permutations = 1;
        for i in 2..=prime_count {
            prime_permutations = (prime_permutations * i) % MOD;
        }
        
        // 计算非素数的排列数量

        let mut non_prime_permutations = 1;
        for i in 2..=non_prime_count {
            non_prime_permutations = (non_prime_permutations * i) % MOD;
        }
        
        // 计算最终的答案

        let result = (prime_permutations * non_prime_permutations) % MOD;
        result as i32

    }
}
