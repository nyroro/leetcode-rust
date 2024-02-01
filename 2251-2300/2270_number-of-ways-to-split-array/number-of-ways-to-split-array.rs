
impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let s: i64 = nums.iter().map(|&x| x as i64).sum();
        let mut prefix_sum: i64 = 0;
        let mut result: i32 = 0;
        let mut prefix_sums: Vec<i64> = Vec::new();

        for &num in nums.iter() {
            prefix_sum += num as i64;
            prefix_sums.push(prefix_sum);
        }

        for i in 0..nums.len() {
            let left = prefix_sums[i];
            let mid = prefix_sums[i];
            let right = s - prefix_sums[i];

            if left > right {
                break;
            }

            let left_bound = match prefix_sums.binary_search(&left) {
                Ok(idx) => idx,
                Err(idx) => idx,
            };

            let right_bound = match prefix_sums.binary_search(&right) {
                Ok(idx) => idx,
                Err(idx) => idx,
            };

            result += (right_bound - left_bound) as i32;
        }

        result

    }
}
