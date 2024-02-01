
impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        // 辅助函数用于判断一个数是否为质数

        fn is_prime(num: i32) -> bool {
            if num <= 1 {
                return false;
            }
            let mut i = 2;
            while i * i <= num {
                if num % i == 0 {
                    return false;
                }
                i += 1;
            }
            true

        }

        let mut min_diff = i32::MAX;
        let mut result = vec![-1, -1];

        for i in left..=right {
            if is_prime(i) {
                for j in (i + 1)..=right {
                    if is_prime(j) {
                        let diff = j - i;
                        if diff < min_diff {
                            min_diff = diff;
                            result = vec![i, j];
                        }
                    }
                }
            }
        }

        result

    }
}
