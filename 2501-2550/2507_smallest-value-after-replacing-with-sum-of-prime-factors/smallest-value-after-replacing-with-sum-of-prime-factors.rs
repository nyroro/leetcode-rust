
// 导入所需的数学库

use std::collections::HashMap;

impl Solution {
    pub fn smallest_value(n: i32) -> i32 {
        // 计算n的所有质因数的和

        fn sum_of_prime_factors(mut n: i32) -> i32 {
            let mut sum = 0;
            let mut i = 2;
            while i * i <= n {
                while n % i == 0 {
                    sum += i;
                    n /= i;
                }
                i += 1;
            }
            if n > 1 {
                sum += n;
            }
            sum

        }
        
        // 不断替换n，直到n不再改变

        let mut seen = HashMap::new();
        let mut num = n;
        while !seen.contains_key(&num) {
            seen.insert(num, true);
            num = sum_of_prime_factors(num);
        }
        
        num

    }
}
