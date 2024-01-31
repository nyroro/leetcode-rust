


impl Solution {
    pub fn minimum_cost(source: String, target: String, original: Vec<char>, changed: Vec<char>, cost: Vec<i32>) -> i64 {
        let inf = 1e18 as i64;
        let mut c = vec![vec![inf; 26]; 26];
        let m = original.len();

        for i in 0..26 {
            c[i][i] = 0;
        }

        for i in 0..m {
            let orig_idx = (original[i] as u8 - b'a') as usize;
            let changed_idx = (changed[i] as u8 - b'a') as usize;
            c[orig_idx][changed_idx] = c[orig_idx][changed_idx].min(cost[i] as i64);
        }

        for k in 0..26 {
            for i in 0..26 {
                for j in 0..26 {
                    c[i][j] = c[i][j].min(c[i][k] + c[k][j]);
                }
            }
        }

        let mut ans = 0;
        let n = source.len();

        for (s, t) in source.chars().zip(target.chars()) {
            let s_idx = (s as u8 - b'a') as usize;
            let t_idx = (t as u8 - b'a') as usize;
            ans += c[s_idx][t_idx];
            if ans >= inf {
                return -1;
            }
        }

        ans

    }
}
