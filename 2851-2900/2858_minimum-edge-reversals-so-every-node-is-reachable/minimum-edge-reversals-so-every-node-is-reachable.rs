
use std::collections::HashMap;

struct Graph {
    outgoing: HashMap<i32, Vec<i32>>,
    incoming: HashMap<i32, Vec<i32>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            outgoing: HashMap::new(),
            incoming: HashMap::new(),
        }
    }

    fn add_edge(&mut self, u: i32, v: i32) {
        self.outgoing.entry(u).or_insert(vec![]).push(v);
        self.incoming.entry(v).or_insert(vec![]).push(u);
    }
}

impl Solution {
    pub fn min_edge_reversals(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph = Graph::new();
        for edge in &edges {
            graph.add_edge(edge[0], edge[1]);
        }

        let mut ans_diff = vec![0; n as usize];
        let mut root_revs = 0;

        fn dfs(node: i32, parent: Option<i32>, depth: i32, revs: i32, ans_diff: &mut Vec<i32>, graph: &Graph, root_revs: &mut i32) {
            ans_diff[node as usize] = -revs + (depth - revs);
            for &child in graph.outgoing.get(&node).unwrap_or(&vec![]) {
                if Some(child) == parent {
                    continue;
                }
                dfs(child, Some(node), depth + 1, revs, ans_diff, graph, root_revs);
            }
            for &child in graph.incoming.get(&node).unwrap_or(&vec![]) {
                if Some(child) == parent {
                    continue;
                }
                *root_revs += 1;
                dfs(child, Some(node), depth + 1, revs + 1, ans_diff, graph, root_revs);
            }
        }

        dfs(0, None, 0, 0, &mut ans_diff, &graph, &mut root_revs);

        let ans: Vec<i32> = ans_diff.iter().map(|&diff| root_revs + diff).collect();
        ans

    }
}
