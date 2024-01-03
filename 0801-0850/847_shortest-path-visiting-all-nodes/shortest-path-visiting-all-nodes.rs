
impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        let target = (1 << n) - 1;
        let mut dp = vec![vec![std::i32::MAX; n]; 1 << n];
        let mut queue = std::collections::VecDeque::new();

        for i in 0..n {
            dp[1 << i][i] = 0;
            queue.push_back((1 << i, i));
        }

        while let Some((mask, u)) = queue.pop_front() {
            let dist = dp[mask][u];
            if mask == target {
                return dist;
            }

            for &v in &graph[u] {
                let new_mask = mask | (1 << v);
                if dist + 1 < dp[new_mask][v as usize] {
                    dp[new_mask][v as usize] = dist + 1;
                    queue.push_back((new_mask, v as usize));
                }
            }
        }

        -1

    }
}
