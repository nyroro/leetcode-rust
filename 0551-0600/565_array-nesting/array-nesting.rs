
impl Solution {
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let mut max_length = 0;
        let n = nums.len();

        for i in 0..n {
            if nums[i] != -1 {
                let mut start = nums[i];
                let mut length = 0;

                while nums[start as usize] != -1 {
                    let temp = start;
                    start = nums[start as usize];
                    nums[temp as usize] = -1;
                    length += 1;
                }

                max_length = max_length.max(length);
            }
        }

        max_length

    }
}
