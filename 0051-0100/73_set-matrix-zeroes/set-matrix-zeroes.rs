
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut rows = vec![false; m];
        let mut cols = vec![false; n];
        
        // 遍历矩阵，记录需要设置为0的行和列

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    rows[i] = true;
                    cols[j] = true;
                }
            }
        }
        
        // 根据标记数组设置矩阵中的元素

        for i in 0..m {
            for j in 0..n {
                if rows[i] || cols[j] {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}
