
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn network_becomes_idle(edges: Vec<Vec<i32>>, patience: Vec<i32>) -> i32 {
        let n = patience.len();
        let mut graph = vec![vec![]; n];
        for edge in edges {
            graph[edge[0] as usize].push(edge[1] as usize);
            graph[edge[1] as usize].push(edge[0] as usize);
        }

        let mut distance = vec![std::i32::MAX; n];
        distance[0] = 0;

        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, 0)));

        while let Some(Reverse((dist, node))) = heap.pop() {
            if dist > distance[node] {
                continue;
            }
            for &next in &graph[node] {
                let next_dist = dist + 1;
                if next_dist < distance[next] {
                    distance[next] = next_dist;
                    heap.push(Reverse((next_dist, next)));
                }
            }
        }

        let mut idle_time = 0;
        for i in 1..n {
            if patience[i] >= 2 * distance[i] {
                idle_time = idle_time.max(2 * distance[i] + 1);
            } else {
                let m = (2 * distance[i] - 1) / patience[i];
                let last_message = m * patience[i];
                let last_receive = last_message + 2 * distance[i] + 1;
                idle_time = idle_time.max(last_receive);
            }
        }

        idle_time

    }
}
