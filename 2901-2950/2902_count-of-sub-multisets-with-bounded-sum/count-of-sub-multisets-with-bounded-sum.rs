
use std::collections::HashMap;

impl Solution {
    pub fn count_sub_multisets(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        const MOD: i64 = 1e9 as i64 + 7;
        let mut dp: Vec<i64> = vec![0; 20001];
        let mut m: HashMap<i64, i64> = HashMap::new();
        dp[0] = 1;
        m.insert(0, 1);
        
        for x in nums {
            *m.entry(x as i64).or_insert(0) += 1;
        }
        
        let mult = m.get(&0).cloned().unwrap_or(0);
        m.remove(&0);
        
        for (a, c) in m.iter() {
            let mut help: Vec<i64> = vec![0; 20001];
            for i in 1..=20000 {
                let mut sum = 0;
                if i - a >= 0 {
                    sum += dp[(i - a) as usize];
                    sum += help[(i - a) as usize];
                }
                if i - a * (c + 1) >= 0 {
                    sum -= dp[(i - a * (c + 1)) as usize];
                }
                help[i as usize] = (sum + MOD) % MOD;
            }
            for i in 0..=20000 {
                dp[i as usize] += help[i as usize];
                dp[i as usize] %= MOD;
            }
        }
        
        let mut sum = 0;
        for i in l..=r {
            sum += dp[i as usize];
            sum %= MOD;
        }
        
        (sum * mult % MOD) as i32

    }
}
