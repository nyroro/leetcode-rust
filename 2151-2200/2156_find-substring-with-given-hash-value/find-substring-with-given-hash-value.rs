
impl Solution {
    pub fn sub_str_hash(s: String, power: i32, modulo: i32, k: i32, hash_value: i32) -> String {
        let s_chars: Vec<char> = s.chars().collect();
        let n = s_chars.len();
        let mut sum: i64 = 0;
        let mut p: i64 = 1;

        for i in 0..k {
            sum = (sum + ((s_chars[n - k as usize + i as usize] as i64 - 'a' as i64 + 1) * p) % modulo as i64) % modulo as i64;
            if i != k - 1 {
                p = (p * power as i64) % modulo as i64;
            }
        }

        let mut res: usize = n - k as usize;

        for i in (k as usize..n).rev() {
            sum = (sum - ((s_chars[i] as i64 - 'a' as i64 + 1) * (p % modulo as i64)) % modulo as i64 + modulo as i64) % modulo as i64;
            sum = (sum * (power as i64 % modulo as i64)) % modulo as i64;
            sum = (sum + (s_chars[i - k as usize] as i64 - 'a' as i64 + 1)) % modulo as i64;
            while sum < 0 {
                sum += modulo as i64;
            }
            if sum == hash_value as i64 {
                res = i - k as usize;
            }
        }

        s_chars.iter().skip(res).take(k as usize).collect()
    }
}
