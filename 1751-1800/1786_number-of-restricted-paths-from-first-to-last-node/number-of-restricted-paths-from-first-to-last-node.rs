
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

impl Solution {
    pub fn count_restricted_paths(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph: HashMap<usize, Vec<(usize, i32)>> = HashMap::new();
        
        for edge in edges {
            let (u, v, w) = (edge[0] as usize, edge[1] as usize, edge[2]);
            graph.entry(u).or_insert(Vec::new()).push((v, w));
            graph.entry(v).or_insert(Vec::new()).push((u, w));
        }
        
        let mut dist: Vec<i32> = vec![std::i32::MAX; n + 1];
        let mut count: Vec<i64> = vec![0; n + 1];
        let mut min_heap = BinaryHeap::new();
        
        dist[n] = 0;
        count[n] = 1;
        min_heap.push(Reverse((0, n)));
        
        while let Some(Reverse((d, node))) = min_heap.pop() {
            if d > dist[node] {
                continue;
            }
            for &(nei, weight) in &graph[&node] {
                if dist[nei] > d + weight {
                    dist[nei] = d + weight;
                    min_heap.push(Reverse((dist[nei], nei)));
                }
            }
        }
        
        let mut stack: Vec<usize> = (1..=n).collect();
        stack.sort_by_key(|&x| dist[x]);
        
        for &node in &stack {
            for &(nei, weight) in &graph[&node] {
                if dist[node] > dist[nei] {
                    count[node] += count[nei];
                    count[node] %= 1_000_000_007;
                }
            }
        }
        
        count[1] as i32

    }
}
