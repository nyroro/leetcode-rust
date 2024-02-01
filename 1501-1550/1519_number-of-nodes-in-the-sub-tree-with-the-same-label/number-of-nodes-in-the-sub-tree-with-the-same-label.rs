
use std::collections::HashMap;

struct TreeNode {
    label: char,
    children: Vec<TreeNode>,
}

impl TreeNode {
    fn new(label: char) -> TreeNode {
        TreeNode { label, children: vec![] }
    }
}

impl Solution {
    pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        let mut result = vec![0; n as usize];
        let mut tree = vec![TreeNode::new(' '); n as usize];
        let mut adjacency_list: HashMap<i32, Vec<i32>> = HashMap::new();

        for edge in edges {
            adjacency_list.entry(edge[0]).or_insert(vec![]).push(edge[1]);
            adjacency_list.entry(edge[1]).or_insert(vec![]).push(edge[0]);
        }

        fn dfs(node: i32, parent: i32, labels: &Vec<char>, tree: &mut Vec<TreeNode>, result: &mut Vec<i32>) -> HashMap<char, i32> {
            let label = labels[node as usize];
            let mut label_count = HashMap::new();
            label_count.insert(label, 1);

            for &child in adjacency_list.get(&node).unwrap_or(&vec![]) {
                if child != parent {
                    let child_label_count = dfs(child, node, labels, tree, result);
                    for (k, v) in child_label_count {
                        *label_count.entry(k).or_insert(0) += v;
                    }
                }
            }

            result[node as usize] = *label_count.get(&label).unwrap_or(&0);
            label_count

        }

        dfs(0, -1, &labels.chars().collect(), &mut tree, &mut result);
        result

    }
}
