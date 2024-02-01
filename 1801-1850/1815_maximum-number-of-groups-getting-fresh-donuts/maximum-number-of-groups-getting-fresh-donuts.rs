
use std::collections::HashMap;

impl Solution {
    pub fn max_happy_groups(batch_size: i32, groups: Vec<i32>) -> i32 {
        let mut f: HashMap<i64, i32> = HashMap::new();
        let size = batch_size as usize;
        let mut ans = 0;
        let mut state: i64 = 0;
        
        for g in &groups {
            let i = *g % size as i32;
            if i == 0 {
                ans += 1;
            } else {
                state += 1 << (i * 5);
            }
        }
        
        ans += dfs(state, 0, &mut f, size);
        ans

    }
    
    fn dfs(state: i64, mod_val: i32, f: &mut HashMap<i64, i32>, size: usize) -> i32 {
        if let Some(&v) = f.get(&state) {
            return v;
        }
        
        let mut res = 0;
        for i in 1..size {
            if (state >> (i * 5)) & 31 != 0 {
                let t = dfs(state - (1 << (i * 5)), (mod_val + i as i32) % size as i32, f, size);
                res = res.max(t + if mod_val == 0 { 1 } else { 0 });
            }
        }
        
        f.insert(state, res);
        res

    }
}
