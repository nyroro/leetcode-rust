
impl Solution {
    pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let mut answer = vec![vec![0; n]; m];
        
        for i in 0..m {
            for j in 0..n {
                let mut sum = 0;
                for r in (i - k as usize).max(0)..=(i + k as usize).min(m - 1) {
                    for c in (j - k as usize).max(0)..=(j + k as usize).min(n - 1) {
                        sum += mat[r][c];
                    }
                }
                answer[i][j] = sum;
            }
        }
        
        answer

    }
}
