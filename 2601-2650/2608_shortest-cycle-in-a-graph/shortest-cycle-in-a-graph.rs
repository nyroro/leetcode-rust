


use std::collections::VecDeque;

impl Solution {
    pub fn find_shortest_cycle(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut ans = std::i32::MAX;
        let mut adj: Vec<Vec<i32>> = vec![vec![]; n as usize];

        for e in &edges {
            adj[e[0] as usize].push(e[1]);
            adj[e[1] as usize].push(e[0]);
        }

        for i in 0..n {
            let mut visited = vec![std::i32::MAX; n as usize];
            let mut parent = vec![-1; n as usize];
            visited[i as usize] = 0;
            let mut q = VecDeque::new();
            q.push_back(i);

            while let Some(src) = q.pop_front() {
                for &nbr in &adj[src as usize] {
                    if visited[nbr as usize] == std::i32::MAX {
                        visited[nbr as usize] = visited[src as usize] + 1;
                        parent[nbr as usize] = src;
                        q.push_back(nbr);
                    } else {
                        if parent[src as usize] == nbr || parent[nbr as usize] == src {
                            continue;
                        }
                        ans = ans.min((visited[src as usize] + visited[nbr as usize] + 1).abs());
                    }
                }
            }
        }

        if ans == std::i32::MAX {
            -1

        } else {
            ans

        }
    }
}
