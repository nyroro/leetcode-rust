


impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut sum = 0i64;
        let mut min_abs = i32::MAX;
        let mut negative_count = 0;

        for row in &matrix {
            for &num in row {
                let abs_num = num.abs() as i64;
                sum += abs_num;
                min_abs = min_abs.min(abs_num as i32);
                if num < 0 {
                    negative_count += 1;
                }
            }
        }

        sum - 2 * (negative_count % 2) * min_abs as i64

    }
}
