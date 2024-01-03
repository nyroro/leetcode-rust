
impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut lengths = vec![1; n];
        let mut counts = vec![1; n];
        let mut max_length = 1;

        for i in 0..n {
            for j in 0..i {
                if nums[i] > nums[j] {
                    if lengths[j] + 1 > lengths[i] {
                        lengths[i] = lengths[j] + 1;
                        counts[i] = counts[j];
                    } else if lengths[j] + 1 == lengths[i] {
                        counts[i] += counts[j];
                    }
                }
            }
            max_length = max_length.max(lengths[i]);
        }

        let mut result = 0;
        for i in 0..n {
            if lengths[i] == max_length {
                result += counts[i];
            }
        }

        result

    }
}
