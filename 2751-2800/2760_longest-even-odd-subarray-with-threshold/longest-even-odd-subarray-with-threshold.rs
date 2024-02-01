
impl Solution {
    pub fn longest_alternating_subarray(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut ans = 0;
        let n = nums.len();
        
        for i in 0..n {
            if nums[i] % 2 == 0 && nums[i] <= threshold {
                let mut e = i + 1;
                while e < n && nums[e] <= threshold && nums[e] % 2 != nums[e - 1] % 2 {
                    e += 1;
                }
                ans = ans.max(e - i);
            }
        }
        
        ans as i32

    }
}
