


impl Solution {
    pub fn count_anagrams(s: String) -> i64 {
        let mut ways: i64 = 1;
        
        for word in s.split_whitespace() {
            ways = (ways * Solution::calculate_display(word)) % 1_000_000_007;
        }
        
        ways

    }
    
    fn calculate_display(word: &str) -> i64 {
        let mut fact = Solution::calculate_factorial(word.len() as i64);
        let mut hashmap = std::collections::HashMap::new();
        
        for w in word.chars() {
            *hashmap.entry(w).or_insert(0) += 1;
        }
        
        for val in hashmap.values() {
            fact = (fact * Solution::calculate_inverse_factorial(*val as i64)) % 1_000_000_007;
        }
        
        fact

    }
    
    fn calculate_factorial(n: i64) -> i64 {
        (1..=n).fold(1, |acc, x| (acc * x) % 1_000_000_007)
    }
    
    fn calculate_inverse_factorial(n: i64) -> i64 {
        (2..=n).fold(1, |acc, x| {
            let inv = Solution::mod_inverse(x, 1_000_000_007);
            (acc * inv) % 1_000_000_007

        })
    }
    
    fn mod_inverse(a: i64, m: i64) -> i64 {
        let mut m0 = m;
        let mut a0 = a;
        let mut t = 0;
        let mut q = 0;
        let mut x0 = 0;
        let mut x1 = 1;
        
        if m == 1 {
            return 0;
        }
        
        while a0 > 1 {
            q = a0 / m0;
            t = m0;
            m0 = a0 % m0;
            a0 = t;
            t = x0;
            x0 = x1 - q * x0;
            x1 = t;
        }
        
        if x1 < 0 {
            x1 += m;
        }
        
        x1

    }
}
