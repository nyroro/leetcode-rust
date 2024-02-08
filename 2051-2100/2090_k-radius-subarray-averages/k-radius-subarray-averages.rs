


impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let mut avgs = vec![-1; n]; // Initialize the result vector with -1


        let (mut i, mut j) = (0, 0);
        let k = k as usize; // Convert k to usize


        let mut sum: i64 = 0;

        while j < n {
            sum += nums[j] as i64;

            if j - i + 1 < 2 * k + 1 {
                j += 1;
            } else if j - i + 1 == 2 * k + 1 {
                avgs[i + k] = (sum / (2 * k as i64 + 1)) as i32; // Use k as usize for indexing

                sum -= nums[i] as i64;
                i += 1;
                j += 1;
            } else {
                break;
            }
        }

        avgs

    }
}
