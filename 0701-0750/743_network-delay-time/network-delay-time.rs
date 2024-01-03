
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        
        // Step 1: Initialize dist array

        let mut dist = vec![std::i32::MAX; n + 1];
        dist[k] = 0;
        
        // Step 2: Create priority queue

        let mut pq = BinaryHeap::new();
        pq.push(Reverse((0, k)));
        
        // Step 3: Create graph

        let mut graph = vec![Vec::new(); n + 1];
        for time in times {
            let u = time[0] as usize;
            let v = time[1] as usize;
            let w = time[2];
            graph[u].push((v, w));
        }
        
        // Step 4: Dijkstra's algorithm

        while let Some(Reverse((time, node))) = pq.pop() {
            if time > dist[node] {
                continue;
            }
            
            for &(neighbor, neighbor_time) in &graph[node] {
                let new_time = time + neighbor_time;
                if new_time < dist[neighbor] {
                    dist[neighbor] = new_time;
                    pq.push(Reverse((new_time, neighbor)));
                }
            }
        }
        
        // Step 5: Find maximum time

        let max_time = dist.iter().skip(1).max();
        if let Some(&time) = max_time {
            if time == std::i32::MAX {
                return -1;
            } else {
                return time;
            }
        }
        
        -1

    }
}
