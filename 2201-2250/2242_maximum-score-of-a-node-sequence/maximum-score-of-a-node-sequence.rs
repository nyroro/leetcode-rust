
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Eq, PartialEq, Clone)]
struct Pair {
    node: usize,
    score: i32,
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}



impl Solution {
    pub fn maximum_score(scores: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = scores.len();
        let mut edge_list: Vec<BinaryHeap<Reverse<Pair>>> = vec![BinaryHeap::new(); n];

        for edge in &edges {
            edge_list[edge[0] as usize].push(Reverse(Pair { node: edge[1] as usize, score: -scores[edge[1] as usize] }));
            edge_list[edge[1] as usize].push(Reverse(Pair { node: edge[0] as usize, score: -scores[edge[0] as usize] }));
        }

        let mut best = -1;

        for e in &edges {
            let mut n0 = Vec::new();
            let mut n1 = Vec::new();

            while let Some(x) = edge_list[e[0] as usize].pop() {
                n0.push(x.0);
                if n0.len() >= 3 {
                    break;
                }
            }

            while let Some(x) = edge_list[e[1] as usize].pop() {
                n1.push(x.0);
                if n1.len() >= 3 {
                    break;
                }
            }

            let m0: Vec<Pair> = n0.iter().filter(|x| x.node != e[1] as usize).cloned().collect();
            let m1: Vec<Pair> = n1.iter().filter(|x| x.node != e[0] as usize).cloned().collect();

            let mut score = scores[e[0] as usize] + scores[e[1] as usize];

            if !m0.is_empty() && !m1.is_empty() {
                if m0[0].node == m1[0].node {
                    if m0.len() > 1 && m1.len() == 1 {
                        score += scores[m0[1].node] + scores[m1[0].node];
                        best = best.max(score);
                    } else if m0.len() == 1 && m1.len() > 1 {
                        score += scores[m0[0].node] + scores[m1[1].node];
                        best = best.max(score);
                    } else if m0.len() > 1 && m1.len() > 1 {
                        score += scores[m0[0].node] + std::cmp::max(scores[m0[1].node], scores[m1[1].node]);
                        best = best.max(score);
                    }
                } else {
                    score += scores[m0[0].node] + scores[m1[0].node];
                    best = best.max(score);
                }
            }

            for n in n0 {
                edge_list[e[0] as usize].push(Reverse(n));
            }

            for n in n1 {
                edge_list[e[1] as usize].push(Reverse(n));
            }
        }

        best

    }
}
