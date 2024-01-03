
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        // 辅助函数：计算树的高度

        fn height(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match node {
                Some(n) => {
                    let n = n.borrow();
                    1 + height(&n.left).max(height(&n.right))
                }
                None => 0,
            }
        }

        // 辅助函数：填充二维数组

        fn fill(node: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<Vec<String>>, row: usize, left: usize, right: usize) {
            if let Some(n) = node {
                let n = n.borrow();
                let mid = (left + right) / 2;
                res[row][mid] = n.val.to_string();
                fill(&n.left, res, row + 1, left, mid);
                fill(&n.right, res, row + 1, mid, right);
            }
        }

        let h = height(&root);
        let m = h as usize;
        let n = (1 << h) - 1;
        let mut res = vec![vec!["".to_string(); n]; m];

        fill(&root, &mut res, 0, 0, n);
        res

    }
}
