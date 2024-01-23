
impl Solution {
    pub fn max_subarrays(nums: Vec<i32>) -> i32 {
        let mut t = nums[0];
        for &k in nums.iter().skip(1) {
            t &= k;
        }
        let mut ret = 1;
        let mut now = 0x7fffffff;

        let mut arr = vec![0; nums.len()];
        for i in (0..nums.len()).rev() {
            arr[i] = nums[i] & now;
            now &= nums[i];
        }
        now = 0x7fffffff;

        for i in 0..nums.len() {
            now &= nums[i];
            if now == 0 && i + 1 < nums.len() && arr[i + 1] <= t {
                ret += 1;
                now = 0x7fffffff;
            } else if now == t && i + 1 < nums.len() && arr[i + 1] == 0 {
                ret += 1;
                now = 0x7fffffff;
            }
        }
        ret

    }
}
