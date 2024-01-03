
impl Solution {
    pub fn min_number_of_semesters(n: i32, relations: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = n as usize;
        let mut graph = vec![0; n];
        let mut in_degree = vec![0; n];
        
        for relation in relations {
            let prev = relation[0] as usize - 1;
            let next = relation[1] as usize - 1;
            graph[next] |= 1 << prev;
            in_degree[next] += 1;
        }
        
        let mut dp = vec![n as i32; 1 << n];
        dp[0] = 0;
        
        for state in 0..1 << n {
            let mut can_take = 0;
            for i in 0..n {
                if (state & graph[i]) == graph[i] && (state & (1 << i)) == 0 {
                    can_take |= 1 << i;
                }
            }
            can_take &= !(state | graph.iter().enumerate().filter(|&(i, _)| (state & (1 << i)) != 0).fold(0, |acc, (_, &val)| acc | val));
            
            let mut subset = can_take as usize; // Convert subset to usize

            while subset > 0 {
                if (subset.count_ones() as i32) <= k {
                    dp[state | subset] = dp[state | subset].min(dp[state] + 1);
                }
                subset = (subset - 1) & can_take as usize; // Convert can_take to usize

            }
        }
        
        dp[(1 << n) - 1]
    }
}
