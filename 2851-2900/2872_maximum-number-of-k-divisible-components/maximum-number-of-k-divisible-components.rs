


impl Solution {
    pub fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>, k: i32) -> i32 {
        let mut dp: Vec<i64> = vec![0; n as usize];
        let mut nodes: Vec<Vec<i32>> = vec![Vec::new(); n as usize];
        
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            nodes[u].push(v as i32);
            nodes[v].push(u as i32);
        }
        
        let mut ans = 0;
        
        fn dfs(u: usize, p: usize, nodes: &Vec<Vec<i32>>, values: &Vec<i32>, dp: &mut Vec<i64>, k: i32, ans: &mut i32) {
            dp[u] = values[u] as i64;
            
            for &v in &nodes[u] {
                if v as usize == p { continue; }
                
                dfs(v as usize, u, nodes, values, dp, k, ans);
                
                if dp[v as usize] % k as i64 == 0 {
                    *ans += 1;
                } else {
                    dp[u] += dp[v as usize];
                    dp[u] %= k as i64;
                }
            }
        }
        
        dfs(0, 0, &nodes, &values, &mut dp, k, &mut ans);
        (ans + 1) as i32

    }
}
