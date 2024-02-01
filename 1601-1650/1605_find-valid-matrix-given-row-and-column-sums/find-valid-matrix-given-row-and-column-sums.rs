
impl Solution {
    pub fn restore_matrix(row_sum: Vec<i32>, col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![0; col_sum.len()]; row_sum.len()];

        for i in 0..row_sum.len() {
            for j in 0..col_sum.len() {
                matrix[i][j] = std::cmp::min(row_sum[i], col_sum[j]);
                row_sum[i] -= matrix[i][j];
                col_sum[j] -= matrix[i][j];
            }
        }

        matrix

    }
}
