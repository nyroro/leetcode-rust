
#[derive(Clone)]
struct TreeNode {
    value: i32,
    children: Vec<usize>,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode {
            value,
            children: Vec::new(),
        }
    }
}

impl Solution {
    pub fn component_value(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut tree: Vec<TreeNode> = vec![TreeNode::new(0); n];
        
        // Build the tree from the given edges

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            tree[u].children.push(v);
            tree[v].children.push(u);
        }
        
        // Calculate the total sum of values in the tree

        let total: i32 = nums.iter().sum();
        
        // Function to perform post-order DFS and calculate the sum of values for each connected component

        fn dfs(u: usize, p: usize, nums: &Vec<i32>, tree: &Vec<TreeNode>, total: i32) -> i32 {
            let mut ans = nums[u];
            for &v in &tree[u].children {
                if v != p {
                    ans += dfs(v, u, nums, tree, total);
                }
            }
            if ans == total {
                return 0;
            } else {
                return ans;
            }
        }
        
        // Iterate through possible component values and return the maximum number of edges that can be deleted

        for cand in 1..=total/2 {
            if total % cand == 0 && dfs(0, n, &nums, &tree, cand) == 0 {
                return total / cand - 1;
            }
        }
        
        return 0;
    }
}
