
impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        // Helper function to generate prime numbers using the Sieve of Eratosthenes

        fn generate_primes(limit: usize) -> Vec<bool> {
            let mut is_prime = vec![true; limit + 1];
            is_prime[0] = false;
            is_prime[1] = false;

            let mut num = 2;
            while num * num <= limit {
                if is_prime[num] {
                    let mut multiple = num * num;
                    while multiple <= limit {
                        is_prime[multiple] = false;
                        multiple += num;
                    }
                }
                num += 1;
            }
            is_prime

        }

        let limit = right.max(2) as usize;
        let primes = generate_primes(limit);

        let mut min_diff = i32::MAX;
        let mut result = vec![-1, -1];

        for num in left..=right {
            if primes[num as usize] {
                let mut temp = num + 1;
                while temp <= right {
                    if primes[temp as usize] {
                        let diff = temp - num;
                        if diff < min_diff {
                            min_diff = diff;
                            result = vec![num, temp];
                        }
                        break;
                    }
                    temp += 1;
                }
            }
        }

        result

    }
}
