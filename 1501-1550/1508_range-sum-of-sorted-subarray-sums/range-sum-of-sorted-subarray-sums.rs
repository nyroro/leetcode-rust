
impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let mut subarray_sums = Vec::new();
        let modulo = 1_000_000_007;

        // Compute the sum of all non-empty continuous subarrays

        for i in 0..nums.len() {
            let mut sum = 0;
            for j in i..nums.len() {
                sum += nums[j];
                subarray_sums.push(sum);
            }
        }

        // Sort the subarray sums in non-decreasing order

        subarray_sums.sort_unstable();

        // Calculate the sum of the numbers from index left to index right

        let mut result = 0;
        for i in left - 1..right {
            result = (result + subarray_sums[i as usize]) % modulo;
        }
        
        result

    }
}
