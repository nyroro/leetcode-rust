
use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let n = edges.len();
        let mut graph: HashMap<i32, i32> = HashMap::new();
        
        for i in 0..n {
            if edges[i] != -1 {
                graph.insert(i as i32, edges[i]);
            }
        }
        
        let mut dist1: Vec<i32> = vec![-1; n];
        let mut dist2: Vec<i32> = vec![-1; n];
        
        Self::bfs(node1, &graph, &mut dist1);
        Self::bfs(node2, &graph, &mut dist2);
        
        let mut min_max_dist = std::i32::MAX;
        let mut result = -1;
        
        for i in 0..n {
            if dist1[i] != -1 && dist2[i] != -1 {
                let max_dist = std::cmp::max(dist1[i], dist2[i]);
                if max_dist < min_max_dist {
                    min_max_dist = max_dist;
                    result = i as i32;
                }
            }
        }
        
        result

    }
    
    fn bfs(start: i32, graph: &HashMap<i32, i32>, dist: &mut Vec<i32>) {
        let mut queue: VecDeque<i32> = VecDeque::new();
        queue.push_back(start);
        dist[start as usize] = 0;
        
        while let Some(node) = queue.pop_front() {
            if let Some(&neighbor) = graph.get(&node) {
                if dist[neighbor as usize] == -1 {
                    dist[neighbor as usize] = dist[node as usize] + 1;
                    queue.push_back(neighbor);
                }
            }
        }
    }
}
