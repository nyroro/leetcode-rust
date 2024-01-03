
impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        
        // 检查重塑是否合法

        if m * n != r as usize * c as usize {
            return mat;
        }
        
        let mut reshaped_mat = vec![vec![0; c as usize]; r as usize];
        let mut row = 0;
        let mut col = 0;
        
        for i in 0..m {
            for j in 0..n {
                reshaped_mat[row][col] = mat[i][j];
                col += 1;
                
                if col == c as usize {
                    col = 0;
                    row += 1;
                }
            }
        }
        
        reshaped_mat

    }
}
