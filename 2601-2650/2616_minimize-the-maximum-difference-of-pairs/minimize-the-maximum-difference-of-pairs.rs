
impl Solution {
    pub fn minimize_max(nums: Vec<i32>, p: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut ret = nums[nums.len() - 1] - nums[0];
        
        let mut l = 0;
        let mut r = ret;
        let n = nums.len();
        
        let gao = |x: i32| -> i32 {
            let mut i = 0;
            let mut cnt = 0;
            while i < n - 1 {
                if nums[i + 1] - nums[i] <= x {
                    cnt += 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            cnt

        };
        
        while l <= r {
            let mid = (l + r) / 2;
            if gao(mid) >= p {
                ret = mid;
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        
        ret

    }
}
