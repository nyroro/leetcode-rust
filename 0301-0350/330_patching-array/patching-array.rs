
impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut patches = 0;
        let mut miss: i64 = 1;
        let mut i = 0;
        let mut index = 0;
        
        while miss <= n as i64 {
            if i < nums.len() && nums[i] as i64 <= miss {
                miss += nums[i] as i64;
                i += 1;
            } else {
                miss += miss;
                patches += 1;
            }
        }
        
        patches

    }
}
