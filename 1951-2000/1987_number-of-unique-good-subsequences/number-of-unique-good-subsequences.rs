
const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn number_of_unique_good_subsequences(binary: String) -> i32 {
        let binary_chars: Vec<char> = binary.chars().collect();
        let n = binary.len();
        let mut ends_with_zero = 0;
        let mut ends_with_one = 0;
        let mut has_zero = false;

        for ch in binary_chars {
            if ch == '0' {
                has_zero = true;
                ends_with_zero = (ends_with_zero + ends_with_one) % MOD;
            } else {
                ends_with_one = (ends_with_zero + ends_with_one + 1) % MOD;
            }
        }

        let result = (ends_with_zero + ends_with_one + if has_zero { 1 } else { 0 }) % MOD;
        result as i32

    }
}
