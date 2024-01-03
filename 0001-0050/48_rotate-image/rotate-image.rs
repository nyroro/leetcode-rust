
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        
        // 先进行矩阵的转置

        for i in 0..n {
            for j in i..n {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }
        
        // 再进行每一行的反转

        for i in 0..n {
            matrix[i].reverse();
        }
    }
}
