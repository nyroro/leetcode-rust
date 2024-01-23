
impl Solution {
    pub fn maximum_top(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() == 1 {
            if k % 2 == 1 {
                return -1;
            } else {
                return nums[0];
            }
        }
        if k == 0 {
            return nums[0];
        }
        let mut ret = -1;
        let mut m = -1;
        let mut k = k;
        for (i, &t) in nums.iter().enumerate() {
            k -= 1;
            m = std::cmp::max(t, m);
            if k == 0 {
                if i + 1 < nums.len() {
                    ret = std::cmp::max(ret, nums[i + 1]);
                } else {
                    ret = std::cmp::max(ret, *nums.iter().take(i).max().unwrap());
                }
            } else {
                ret = std::cmp::max(ret, m);
            }
            if k == 0 {
                break;
            }
        }
        ret

    }
}
