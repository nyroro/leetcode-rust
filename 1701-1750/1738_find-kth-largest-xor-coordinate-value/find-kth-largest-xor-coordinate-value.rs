
impl Solution {
    pub fn kth_largest_value(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut xor_matrix = vec![vec![0; n + 1]; m + 1];
        let mut result = Vec::new();

        for i in 1..=m {
            for j in 1..=n {
                xor_matrix[i][j] = xor_matrix[i - 1][j] ^ xor_matrix[i][j - 1] ^ xor_matrix[i - 1][j - 1] ^ matrix[i - 1][j - 1];
                result.push(xor_matrix[i][j]);
            }
        }

        result.sort_unstable_by(|a, b| b.cmp(a));
        result[k as usize - 1]
    }
}
