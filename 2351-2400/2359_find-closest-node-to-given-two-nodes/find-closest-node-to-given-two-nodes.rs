
use std::collections::HashMap;

impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let n = edges.len();
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        
        for i in 0..n {
            if edges[i] != -1 {
                graph.entry(edges[i]).or_insert(Vec::new()).push(i as i32);
            }
        }
        
        let mut visited: Vec<bool> = vec![false; n];
        let mut dist1: Vec<i32> = vec![-1; n];
        let mut dist2: Vec<i32> = vec![-1; n];
        
        Self::dfs(node1, &graph, &mut visited, &mut dist1);
        visited.iter_mut().for_each(|v| *v = false);
        Self::dfs(node2, &graph, &mut visited, &mut dist2);
        
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
    
    fn dfs(node: i32, graph: &HashMap<i32, Vec<i32>>, visited: &mut Vec<bool>, dist: &mut Vec<i32>) {
        visited[node as usize] = true;
        dist[node as usize] = 0;
        
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if !visited[neighbor as usize] {
                    Self::dfs(neighbor, graph, visited, dist);
                    dist[node as usize] = std::cmp::max(dist[node as usize], 1 + dist[neighbor as usize]);
                }
            }
        }
    }
}
