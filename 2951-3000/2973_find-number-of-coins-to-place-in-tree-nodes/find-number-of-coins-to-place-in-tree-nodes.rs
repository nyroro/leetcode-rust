
struct Tree {
    nodes: usize,
    edges: Vec<Vec<usize>>,
    cost: Vec<i32>,
}

impl Tree {
    fn new(nodes: usize, edges: Vec<Vec<usize>>, cost: Vec<i32>) -> Tree {
        Tree { nodes, edges, cost }
    }

    fn traverse_tree(&self, node: usize, prev_node: usize, ans: &mut Vec<i64>) -> (Vec<i32>, Vec<i32>) {
        let mut positives = Vec::new();
        let mut negatives = Vec::new();

        for &v in &self.edges[node] {
            if v != prev_node {
                let (mut child_positives, mut child_negatives) = self.traverse_tree(v, node, ans);
                positives.append(&mut child_positives);
                negatives.append(&mut child_negatives);
            }
        }

        if self.cost[node] >= 0 {
            positives.push(self.cost[node]);
        } else {
            negatives.push(-self.cost[node]);
        }

        positives.sort();
        negatives.sort();

        while positives.len() > 3 {
            positives.remove(0);
        }

        while negatives.len() > 3 {
            negatives.remove(0);
        }

        let max_value = self.calculate_max_value(&positives, &negatives);
        ans[node] = max_value;

        (positives, negatives)
    }

    fn calculate_max_value(&self, positives: &Vec<i32>, negatives: &Vec<i32>) -> i64 {
        let mut temp_values = positives.clone();
        temp_values.extend(negatives.iter().map(|&x| -x));

        if temp_values.len() < 3 {
            return 1;
        }

        let mut max_product = 0;
        for i in 0..temp_values.len() {
            for j in i + 1..temp_values.len() {
                for k in j + 1..temp_values.len() {
                    max_product = max_product.max(
                        temp_values[i] as i64 * temp_values[j] as i64 * temp_values[k] as i64,
                    );
                }
            }
        }

        max_product

    }
}

impl Solution {
    pub fn placed_coins(edges: Vec<Vec<i32>>, cost: Vec<i32>) -> Vec<i64> {
        let n = cost.len();
        let mut ans = vec![0; n];
        let mut tree_edges = vec![Vec::new(); n];

        for edge in edges {
            tree_edges[edge[0] as usize].push(edge[1] as usize);
            tree_edges[edge[1] as usize].push(edge[0] as usize);
        }

        let tree = Tree::new(n, tree_edges, cost);
        tree.traverse_tree(0, 0, &mut ans);

        ans

    }
}
