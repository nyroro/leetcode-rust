
impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let mut primary_sum = 0;
        let mut secondary_sum = 0;
        
        for i in 0..n {
            primary_sum += mat[i][i];
            secondary_sum += mat[i][n - i - 1];
        }
        
        if n % 2 == 1 {
            primary_sum -= mat[n / 2][n / 2];
        }
        
        primary_sum + secondary_sum

    }
}
