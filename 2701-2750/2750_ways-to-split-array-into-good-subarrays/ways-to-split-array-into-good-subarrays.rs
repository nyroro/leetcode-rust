
impl Solution {
    pub fn number_of_good_subarray_splits(nums: Vec<i32>) -> i32 {
        let modulo = 1_000_000_007;
        let ones_indices: Vec<usize> = nums.iter().enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(i, _)| i)
            .collect();

        if ones_indices.len() == 0 {
            return 0;
        }

        let mut result: i64 = 1;
        for i in 0..ones_indices.len() - 1 {
            result *= (ones_indices[i + 1] - ones_indices[i]) as i64;
            result %= modulo;
        }

        result as i32

    }
}
