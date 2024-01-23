
impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let mut ret = 1;
        let (mut x, mut k, mut l) = (0, 0, 0);
        
        for (i, &t) in nums.iter().enumerate() {
            while l < i && (x & t) != 0 {
                x -= nums[l];
                l += 1;
                k -= 1;
            }
            x |= t;
            k += 1;
            
            ret = ret.max(k);
        }
        
        ret

    }
}
