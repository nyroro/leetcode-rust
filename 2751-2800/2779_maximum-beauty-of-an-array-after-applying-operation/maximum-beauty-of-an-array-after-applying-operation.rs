
impl Solution {
    pub fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort(); // 对数组进行排序


        if nums[0] + k >= nums[nums.len() - 1] - k {
            return nums.len() as i32; // 如果最小值加上k大于等于最大值减去k，则整个数组都是连续子序列

        }

        let mut x = nums[0] + 2 * k;
        let mut l = 0;
        let mut y = 1;
        let mut ret = 1;

        for i in 1..nums.len() {
            if nums[i] <= x {
                y += 1;
                ret = ret.max(y);
            } else {
                y += 1;
                while l < i && nums[i] > x {
                    l += 1;
                    x = nums[l] + 2 * k;
                    y -= 1;
                }
                ret = ret.max(y);
            }
        }

        ret as i32

    }
}
