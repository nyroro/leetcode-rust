
impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut diff_cumulative = vec![0i64];
        let mut sum: i64 = 0;

        // Calculate the cumulative sum of differences

        for diff in differences {
            sum = sum + diff as i64;  // Convert diff to i64 to ensure type compatibility

            diff_cumulative.push(sum);
        }

        // Find the minimum and maximum values in the cumulative sum vector

        let min_diff = *diff_cumulative.iter().min().unwrap();
        let max_diff = *diff_cumulative.iter().max().unwrap();

        // Calculate the number of possible hidden sequences

        let result = i64::max(0, upper as i64 - lower as i64 - (max_diff - min_diff) + 1);

        result

    }
}
