
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct Graph {
    adj_list: Vec<Vec<(usize, i32)>>,
    visited: Vec<bool>,
    distances: Vec<i32>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            adj_list: vec![Vec::new(); n],
            visited: vec![false; n],
            distances: vec![i32::MAX; n],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, weight: i32) {
        self.adj_list[u].push((v, weight + 1));
        self.adj_list[v].push((u, weight + 1));
    }

    fn dijkstra(&mut self, max_moves: i32) -> i32 {
        let mut pq = BinaryHeap::new();
        pq.push(Reverse((0, 0))); // distance, node

        self.distances[0] = 0;

        while let Some(Reverse((cd, u))) = pq.pop() {
            if self.visited[u] {
                continue;
            }
            self.visited[u] = true;

            for &(v, d) in &self.adj_list[u] {
                if cd + d < self.distances[v] {
                    self.distances[v] = cd + d;
                    pq.push(Reverse((self.distances[v], v)));
                }
            }
        }

        self.distances.iter().filter(|&&d| d <= max_moves).count() as i32

    }
}

impl Solution {
    pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
        let n = n as usize;
        let mut graph = Graph::new(n);

        for edge in &edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let weight = edge[2];
            graph.add_edge(u, v, weight);
        }

        let mut ans = graph.dijkstra(max_moves);

        for edge in &edges {
            let da = max_moves - graph.distances[edge[0] as usize];
            let db = max_moves - graph.distances[edge[1] as usize];
            ans += edge[2].min(i32::max(da, 0) + i32::max(db, 0));
        }

        ans

    }
}
