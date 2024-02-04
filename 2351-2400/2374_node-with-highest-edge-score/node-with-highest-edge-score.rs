
use std::collections::HashMap;

impl Solution {
    pub fn edge_score(edges: Vec<i32>) -> i32 {
        let n = edges.len();
        let mut edge_scores: HashMap<i32, i64> = HashMap::new();

        for i in 0..n {
            let score = edge_scores.entry(edges[i]).or_insert(0);
            *score += i as i64;
        }

        let mut max_score = 0;
        let mut max_node = 0;

        for (node, score) in edge_scores {
            if score > max_score || (score == max_score && node < max_node) {
                max_score = score;
                max_node = node;
            }
        }

        max_node

    }
}
