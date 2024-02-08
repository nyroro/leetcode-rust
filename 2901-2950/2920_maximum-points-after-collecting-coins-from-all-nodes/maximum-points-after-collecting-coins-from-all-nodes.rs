
#[derive(Clone)]
struct TreeNode {
    coins: i32,
    children: Vec<usize>,
}

impl TreeNode {
    fn new(coins: i32) -> TreeNode {
        TreeNode { coins, children: Vec::new() }
    }
}

impl Solution {
    pub fn maximum_points(edges: Vec<Vec<i32>>, coins: Vec<i32>, k: i32) -> i32 {
        let n = coins.len();
        let mut tree = vec![TreeNode::new(0); n];

        // Build the tree

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            tree[u].children.push(v);
            tree[v].children.push(u);
        }

        // Dynamic programming function

        let mut dp = vec![vec![-1; 15]; n];
        Solution::helper(&tree, &coins, k, &mut dp, 0, n, -1, 0)
    }

    fn helper(tree: &Vec<TreeNode>, coins: &Vec<i32>, k: i32, dp: &mut Vec<Vec<i32>>, curr: usize, n: usize, prev: i32, decay: i32) -> i32 {
        if decay >= 14 {
            return 0;
        }
        if dp[curr as usize][decay as usize] != -1 {
            return dp[curr as usize][decay as usize];
        }

        let mut option1 = ((coins[curr] >> decay) - k);
        let mut option2 = ((coins[curr] >> decay) / 2);
        for &next in &tree[curr].children {
            if next as i32 == prev {
                continue;
            }
            option1 += Solution::helper(tree, coins, k, dp, next, n, curr as i32, decay);
            option2 += Solution::helper(tree, coins, k, dp, next, n, curr as i32, decay + 1);
        }
        dp[curr as usize][decay as usize] = std::cmp::max(option1, option2);
        dp[curr as usize][decay as usize]
    }
}
