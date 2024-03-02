
struct TreeNode {
    index: usize,
    price: i32,
    children: Vec<usize>,
}

impl TreeNode {
    fn new(index: usize, price: i32) -> TreeNode {
        TreeNode {
            index,
            price,
            children: Vec::new(),
        }
    }
}

impl Solution {
    pub fn max_output(n: i32, edges: Vec<Vec<i32>>, price: Vec<i32>) -> i64 {
        let n = n as usize;
        let mut tree: Vec<TreeNode> = (0..n).map(|i| TreeNode::new(i, price[i as usize])).collect();

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            tree[u].children.push(v);
            tree[v].children.push(u);
        }

        let mut max_sums: Vec<(i64, i64)> = vec![(0, 0); n];
        let mut max_cost = 0;

        fn dfs(node: usize, parent: usize, tree: &Vec<TreeNode>, max_sums: &mut Vec<(i64, i64)>) -> i64 {
            let mut a: Vec<i64> = Vec::new();
            for &child in &tree[node].children {
                if child != parent {
                    let t = dfs(child, node, tree, max_sums);
                    a.push(t);
                }
            }
            a.sort_unstable_by(|a, b| b.cmp(a));
            let (x, y) = (a.get(0).cloned().unwrap_or(0), a.get(1).cloned().unwrap_or(0));
            max_sums[node] = (x, y);
            x + tree[node].price as i64

        }

        fn dfs1(node: usize, parent: usize, tree: &Vec<TreeNode>, max_sums: &mut Vec<(i64, i64)>, max_cost: &mut i64) {
            *max_cost = (*max_cost).max(max_sums[node].0);
            if parent != usize::MAX {
                let mut vm = tree[parent].price as i64;
                if max_sums[parent].0 == max_sums[node].0 + tree[node].price as i64 {
                    vm += max_sums[parent].1;
                } else {
                    vm += max_sums[parent].0;
                }
                if max_sums[node].0 < vm {
                    let tt = max_sums[node].0;
                    max_sums[node].0 = vm;
                    max_sums[node].1 = tt;
                } else if max_sums[node].1 < vm {
                    max_sums[node].1 = vm;
                }
                *max_cost = (*max_cost).max(vm);
            }
            for &child in &tree[node].children {
                if child != parent {
                    dfs1(child, node, tree, max_sums, max_cost);
                }
            }
        }

        let _ = dfs(0, usize::MAX, &tree, &mut max_sums);
        dfs1(0, usize::MAX, &tree, &mut max_sums, &mut max_cost);

        max_cost

    }
}
