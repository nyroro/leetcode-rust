
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut end = 0;
        let mut sum = 0;
        let mut min_len = std::usize::MAX;

        while end < nums.len() {
            sum += nums[end];

            while sum >= target {
                min_len = min_len.min(end - start + 1);
                sum -= nums[start];
                start += 1;
            }

            end += 1;
        }

        if min_len == std::usize::MAX {
            return 0;
        }

        min_len as i32

    }
}
