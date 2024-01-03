
impl Solution {
    pub fn reconstruct_matrix(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        let n = colsum.len();
        let mut result = vec![vec![0; n]; 2];
        let mut upper = upper;
        let mut lower = lower;
        
        for i in 0..n {
            if colsum[i] == 2 {
                result[0][i] = 1;
                result[1][i] = 1;
                upper -= 1;
                lower -= 1;
            } else if colsum[i] == 0 {
                result[0][i] = 0;
                result[1][i] = 0;
            } else {
                if upper >= lower {
                    result[0][i] = 1;
                    result[1][i] = 0;
                    upper -= 1;
                } else {
                    result[0][i] = 0;
                    result[1][i] = 1;
                    lower -= 1;
                }
            }
        }
        
        if upper == 0 && lower == 0 {
            result

        } else {
            vec![]
        }
    }
}
