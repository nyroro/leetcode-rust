
impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut count = 0;
        
        // 计算每一行的高度

        let mut heights = vec![0; n];
        
        for i in 0..m {
            // 更新每一列的高度

            for j in 0..n {
                if mat[i][j] == 1 {
                    heights[j] += 1;
                } else {
                    heights[j] = 0;
                }
            }
            
            // 计算以当前行为底的子矩阵数量

            for j in 0..n {
                let mut min_height = heights[j];
                for k in j..n {
                    min_height = min_height.min(heights[k]);
                    count += min_height;
                }
            }
        }
        
        count

    }
}
