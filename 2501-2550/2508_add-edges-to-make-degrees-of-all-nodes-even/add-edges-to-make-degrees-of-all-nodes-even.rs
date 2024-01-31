
use std::collections::HashMap;

impl Solution {
    pub fn is_possible(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();

        // Build the graph

        for edge in edges {
            let a = edge[0];
            let b = edge[1];
            graph.entry(a).or_insert(Vec::new()).push(b);
            graph.entry(b).or_insert(Vec::new()).push(a);
        }

        let mut odds: Vec<i32> = Vec::new();

        // Find nodes with odd degrees

        for (node, neighbors) in &graph {
            if neighbors.len() % 2 != 0 {
                odds.push(*node);
            }
        }

        if odds.is_empty() {
            return true;
        } else if odds.len() > 4 || odds.len() == 1 || odds.len() == 3 {
            return false;
        } else if odds.len() == 2 {
            let a = odds[0];
            let b = odds[1];
            if !graph[&a].contains(&b) {
                return true;
            }
            for i in 1..=n {
                if !graph.contains_key(&i) || (!graph[&a].contains(&i) && !graph[&b].contains(&i)) {
                    return true;
                }
            }
            return false;
        } else {
            let a = odds[0];
            let b = odds[1];
            let c = odds[2];
            let d = odds[3];
            if !graph[&a].contains(&b) && !graph[&c].contains(&d) {
                return true;
            }
            if !graph[&a].contains(&c) && !graph[&b].contains(&d) {
                return true;
            }
            if !graph[&a].contains(&d) && !graph[&b].contains(&c) {
                return true;
            }
            return false;
        }
    }
}
