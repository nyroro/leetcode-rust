
impl Solution {
    pub fn is_prime(num: i32) -> bool {
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
        return true;
    }

    pub fn find_prime_pairs(n: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        for x in 2..=n {
            let y = n - x;
            if x <= y && Solution::is_prime(x) && Solution::is_prime(y) {
                result.push(vec![x, y]);
            }
        }
        result

    }
}
