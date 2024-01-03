
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut count = vec![0; n + 1];
        let mut result = vec![0; 2];

        for num in nums {
            count[num as usize] += 1;
        }

        for i in 1..=n {
            if count[i] == 2 {
                result[0] = i as i32;
            } else if count[i] == 0 {
                result[1] = i as i32;
            }
        }

        result

    }
}
