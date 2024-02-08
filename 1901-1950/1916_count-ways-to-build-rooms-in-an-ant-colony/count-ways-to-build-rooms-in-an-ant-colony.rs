


impl Solution {
    pub fn ways_to_build_rooms(prev_room: Vec<i32>) -> i32 {
        let n = prev_room.len();
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
        
        for (i, &prev) in prev_room.iter().enumerate() {
            if prev != -1 {
                adj[prev as usize].push(i);
            }
        }
        
        let mut dp: Vec<i64> = vec![0; n];
        
        fn dfs(dp: &mut Vec<i64>, adj: &Vec<Vec<usize>>, ind: usize) -> i64 {
            dp[ind] = 1;
            for &it in &adj[ind] {
                dp[ind] += dfs(dp, adj, it);
            }
            dp[ind]
        }
        
        dfs(&mut dp, &adj, 0);
        
        let mod_val: i64 = 1_000_000_007;
        
        let powmod = |mut a: i64, mut x: i64| -> i64 {
            let mut res = 1;
            while x > 0 {
                if x % 2 == 1 {
                    res = (res * a) % mod_val;
                }
                a = (a * a) % mod_val;
                x /= 2;
            }
            res

        };
        
        let inv = |a: i64| -> i64 {
            powmod(a, mod_val - 2)
        };
        
        let fact = |a: i64| -> i64 {
            let mut res = 1;
            for i in 2..=a {
                res = (res * i) % mod_val;
            }
            res

        };
        
        let mut res = fact(n as i64);
        for &it in &dp {
            res = (res * inv(it)) % mod_val;
        }
        
        res as i32

    }
}
