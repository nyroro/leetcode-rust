
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
struct State {
    prob: f64,
    node: usize,
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.prob.partial_cmp(&self.prob).unwrap()
    }
}

impl Solution {
    pub fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start_node: i32, end_node: i32) -> f64 {
        let n = n as usize;
        let mut graph: Vec<Vec<(usize, f64)>> = vec![Vec::new(); n];
        for i in 0..edges.len() {
            let a = edges[i][0] as usize;
            let b = edges[i][1] as usize;
            let p = succ_prob[i];
            graph[a].push((b, p));
            graph[b].push((a, p));
        }

        let mut pq = BinaryHeap::new();
        let mut dist: Vec<f64> = vec![0.0; n];
        dist[start_node as usize] = 1.0;
        pq.push(State { prob: 1.0, node: start_node as usize });

        while let Some(State { prob, node }) = pq.pop() {
            if node == end_node as usize {
                return prob;
            }
            if prob > dist[node] {
                continue;
            }
            for &(next, next_prob) in &graph[node] {
                let next_prob = prob * next_prob;
                if next_prob > dist[next] {
                    dist[next] = next_prob;
                    pq.push(State { prob: next_prob, node: next });
                }
            }
        }

        0.0

    }
}
