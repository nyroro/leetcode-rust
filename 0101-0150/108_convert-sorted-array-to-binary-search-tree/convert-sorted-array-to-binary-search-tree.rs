
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        
        let mid = nums.len() / 2;
        
        let root = Rc::new(RefCell::new(TreeNode::new(nums[mid])));
        
        let left_nums = nums[..mid].to_vec();
        let right_nums = nums[mid + 1..].to_vec();
        
        root.borrow_mut().left = Solution::sorted_array_to_bst(left_nums);
        root.borrow_mut().right = Solution::sorted_array_to_bst(right_nums);
        
        Some(root)
    }
}
