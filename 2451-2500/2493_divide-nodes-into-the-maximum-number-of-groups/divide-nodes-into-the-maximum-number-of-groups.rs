
use std::collections::VecDeque;

struct Graph {
    nodes: Vec<Vec<i32>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph { nodes: vec![vec![]; n] }
    }

    fn add_edge(&mut self, u: i32, v: i32) {
        self.nodes[u as usize - 1].push(v - 1);
        self.nodes[v as usize - 1].push(u - 1);
    }
}

impl Solution {
    pub fn magnificent_sets(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut graph = Graph::new(n as usize);
        for edge in &edges {
            graph.add_edge(edge[0], edge[1]);
        }

        let mut seen = vec![0; n as usize];
        let mut groups = Vec::new();

        for i in 0..n as usize {
            if seen[i] == 0 {
                seen[i] = 1;
                let mut stack = vec![i];
                groups.push(vec![i]);
                while let Some(u) = stack.pop() {
                    for &v in &graph.nodes[u] {
                        if seen[v as usize] == 0 {
                            seen[v as usize] = seen[u] + 1;
                            stack.push(v as usize);
                            groups.last_mut().unwrap().push(v as usize);
                        } else if seen[u] & 1 == seen[v as usize] & 1 {
                            return -1;
                        }
                    }
                }
            }
        }

        fn bfs(graph: &Graph, x: usize) -> usize {
            let mut ans = 0;
            let mut seen = vec![false; graph.nodes.len()];
            let mut queue = VecDeque::new();
            seen[x] = true;
            queue.push_back(x);
            while !queue.is_empty() {
                ans += 1;
                for _ in 0..queue.len() {
                    let u = queue.pop_front().unwrap();
                    for &v in &graph.nodes[u] {
                        if !seen[v as usize] {
                            seen[v as usize] = true;
                            queue.push_back(v as usize);
                        }
                    }
                }
            }
            ans

        }

        let mut max_groups = 0;
        for group in groups {
            max_groups += group.iter().map(|&x| bfs(&graph, x)).max().unwrap();
        }
        max_groups as i32

    }
}
