
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        fn helper(nums: &[i32]) -> i32 {
            let (mut prev, mut curr) = (0, 0);
            for &num in nums {
                let temp = curr;
                curr = curr.max(prev + num);
                prev = temp;
            }
            curr

        }
        
        if nums.len() == 1 {
            return nums[0];
        }
        
        helper(&nums[0..nums.len() - 1]).max(helper(&nums[1..]))
    }
}
