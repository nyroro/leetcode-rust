
impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut result = vec![vec![0; m]; n];
        
        for i in 0..m {
            for j in 0..n {
                result[j][i] = matrix[i][j];
            }
        }
        
        result

    }
}
