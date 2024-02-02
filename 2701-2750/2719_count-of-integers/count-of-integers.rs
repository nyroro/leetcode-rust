
use std::collections::HashMap;



impl Solution {
    const MOD: i64 = 1_000_000_007;

    pub fn count(num1: String, num2: String, min_sum: i32, max_sum: i32) -> i32 {
        let num1 = format!("{:0>width$}", num1, width = num2.len()); // padding num1 with zeros

        let num1_chars: Vec<char> = num1.chars().collect();
        let num2_chars: Vec<char> = num2.chars().collect();

        let mut cache: HashMap<(usize, i32, bool, bool), i64> = HashMap::new();

        fn dp(
            i: usize,
            s: i32,
            has_lower: bool,
            has_upper: bool,
            num1_chars: &Vec<char>,
            num2_chars: &Vec<char>,
            cache: &mut HashMap<(usize, i32, bool, bool), i64>,
        ) -> i64 {
            if s < 0 {
                return 0;
            }
            if i == num1_chars.len() {
                return if s == 0 { 1 } else { 0 };
            }
            if let Some(&val) = cache.get(&(i, s, has_lower, has_upper)) {
                return val;
            }

            let lower = if has_lower {
                num1_chars[i].to_digit(10).unwrap() as i32

            } else {
                0

            };
            let upper = if has_upper {
                num2_chars[i].to_digit(10).unwrap() as i32

            } else {
                9

            };

            let mut result = 0;
            for d in lower..=upper {
                result += dp(
                    i + 1,
                    s - d,
                    has_lower && d == lower,
                    has_upper && d == upper,
                    num1_chars,
                    num2_chars,
                    cache,
                );
                result %= Solution::MOD;
            }

            cache.insert((i, s, has_lower, has_upper), result);
            result

        }

        let mut total = 0;
        for s in min_sum..=max_sum {
            total += dp(0, s, true, true, &num1_chars, &num2_chars, &mut cache);
            total %= Solution::MOD;
        }

        total as i32

    }
}
