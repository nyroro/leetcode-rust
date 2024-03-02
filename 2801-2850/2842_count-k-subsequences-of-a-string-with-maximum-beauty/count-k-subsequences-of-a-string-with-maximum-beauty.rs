
use std::collections::HashMap;

impl Solution {
    const MOD: i64 = 1_000_000_007;

    fn solve(max_beauty: i32, curr_beauty: i32, char_freq: &Vec<i32>, i: usize, k: i32, dp: &mut HashMap<String, i64>) -> i64 {
        if k == 0 {
            if max_beauty == curr_beauty {
                return 1;
            }
            return 0;
        }
        if i >= char_freq.len() || curr_beauty > max_beauty {
            return 0;
        }
        let curr = format!("{}#{}#{}", i, k - 1, curr_beauty);
        if let Some(&val) = dp.get(&curr) {
            return val;
        }

        // Take it

        let take = (char_freq[i] as i64 * Solution::solve(max_beauty, curr_beauty + char_freq[i], char_freq, i + 1, k - 1, dp)) % Solution::MOD;

        // Don't take it

        let not_take = Solution::solve(max_beauty, curr_beauty, char_freq, i + 1, k, dp) % Solution::MOD;

        let result = (take + not_take) % Solution::MOD;
        dp.insert(curr, result);
        result

    }

    pub fn count_k_subsequences_with_max_beauty(s: String, k: i32) -> i32 {
        let mut freq: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            *freq.entry(c).or_insert(0) += 1;
        }
        if k > freq.len() as i32 {
            return 0;
        }
        let mut char_freq: Vec<i32> = freq.values().cloned().collect();
        char_freq.sort_unstable();

        let mut max_beauty = 0;
        let n = char_freq.len();
        for i in (n - 1).saturating_sub(k as usize - 1)..n {
            max_beauty += char_freq[i];
        }
        let mut dp: HashMap<String, i64> = HashMap::new();
        (Solution::solve(max_beauty, 0, &char_freq, 0, k, &mut dp) % Solution::MOD) as i32

    }
}
