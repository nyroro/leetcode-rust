
impl Solution {
    pub fn sieve_of_eratosthenes(n: i32) -> Vec<bool> {
        let mut primes = vec![true; (n + 1) as usize];
        primes[0] = false;
        primes[1] = false;
        let mut p = 2;
        while p * p <= n {
            if primes[p as usize] {
                let mut i = p * p;
                while i <= n {
                    primes[i as usize] = false;
                    i += p;
                }
            }
            p += 1;
        }
        primes

    }

    pub fn find_prime_pairs(n: i32) -> Vec<Vec<i32>> {
        let primes = Solution::sieve_of_eratosthenes(n);
        let mut result = Vec::new();
        let mut x = 2;
        while x <= n / 2 {
            let y = n - x;
            if primes[x as usize] && primes[y as usize] {
                result.push(vec![x, y]);
            }
            x += 1;
        }
        result

    }
}
