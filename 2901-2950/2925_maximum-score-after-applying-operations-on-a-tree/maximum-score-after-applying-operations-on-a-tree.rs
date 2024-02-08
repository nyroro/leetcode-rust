


impl Solution {
    fn dfs(node: usize, adj: &Vec<Vec<usize>>, dp: &mut Vec<i64>, values: &Vec<i32>) -> (i64, i64) {
        if dp[node] != -1 {
            return (0, 0);
        }
        dp[node] = 0;
        let mut ch_sum = 0;
        let mut min_ch_sum = 0;
        for &it in adj[node].iter() {
            if dp[it] == -1 {
                let next = Solution::dfs(it, adj, dp, values);
                ch_sum += next.0;
                min_ch_sum += next.1;
            }
        }
        let val = values[node] as i64;
        if min_ch_sum == 0 {
            dp[node] = 0;
            return (val, val);
        }
        if min_ch_sum >= val {
            dp[node] = ch_sum;
        } else if val > min_ch_sum {
            dp[node] = ch_sum - min_ch_sum + val;
        }
        (ch_sum + val, std::cmp::min(min_ch_sum, val))
    }

    pub fn maximum_score_after_operations(edges: Vec<Vec<i32>>, values: Vec<i32>) -> i64 {
        let n = values.len();
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
        let mut dp: Vec<i64> = vec![-1; n];
        for edge in edges {
            adj[edge[0] as usize].push(edge[1] as usize);
            adj[edge[1] as usize].push(edge[0] as usize);
        }
        Solution::dfs(0, &adj, &mut dp, &values);
        dp[0]
    }
}
