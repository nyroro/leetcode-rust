
use std::collections::HashMap;

impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut count: Vec<i32> = vec![0; n as usize];
        let mut res: Vec<i32> = vec![0; n as usize];
        let mut subtree: Vec<i32> = vec![0; n as usize];

        for edge in edges {
            let u = edge[0];
            let v = edge[1];
            adj.entry(u).or_insert(Vec::new()).push(v);
            adj.entry(v).or_insert(Vec::new()).push(u);
        }

        Solution::dfs(0, -1, &adj, &mut count, &mut res, &mut subtree);
        Solution::dfs2(0, -1, &adj, &mut count, &mut res, &subtree);

        res

    }

    fn dfs(
        node: i32,
        parent: i32,
        adj: &HashMap<i32, Vec<i32>>,
        count: &mut Vec<i32>,
        res: &mut Vec<i32>,
        subtree: &mut Vec<i32>,
    ) {
        if let Some(children) = adj.get(&node) {
            for &child in children {
                if child == parent {
                    continue;
                }
                Solution::dfs(child, node, adj, count, res, subtree);
                count[node as usize] += count[child as usize];
                res[node as usize] += res[child as usize] + count[child as usize];
                subtree[node as usize] += subtree[child as usize];
            }
        }
        count[node as usize] += 1;
        subtree[node as usize] += 1;
    }

    fn dfs2(
        node: i32,
        parent: i32,
        adj: &HashMap<i32, Vec<i32>>,
        count: &mut Vec<i32>,
        res: &mut Vec<i32>,
        subtree: &Vec<i32>,
    ) {
        if let Some(children) = adj.get(&node) {
            for &child in children {
                if child == parent {
                    continue;
                }
                res[child as usize] = res[node as usize] - count[child as usize] + (count.len() as i32 - count[child as usize]);
                Solution::dfs2(child, node, adj, count, res, subtree);
            }
        }
    }
}
