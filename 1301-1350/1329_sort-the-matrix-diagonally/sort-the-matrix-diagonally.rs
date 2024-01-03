
impl Solution {
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let mut result = vec![vec![0; n]; m];
        
        // 复制原始矩阵到结果矩阵

        for i in 0..m {
            for j in 0..n {
                result[i][j] = mat[i][j];
            }
        }
        
        // 对左上部分的对角线进行排序

        for d in 0..n {
            let mut diagonal = Vec::new();
            let mut x = 0;
            let mut y = d;
            while x < m && y < n {
                diagonal.push(result[x][y]);
                x += 1;
                y += 1;
            }
            diagonal.sort();
            x = 0;
            y = d;
            let mut idx = 0;
            while x < m && y < n {
                result[x][y] = diagonal[idx];
                idx += 1;
                x += 1;
                y += 1;
            }
        }
        
        // 对左下部分的对角线进行排序

        for d in 1..m {
            let mut diagonal = Vec::new();
            let mut x = d;
            let mut y = 0;
            while x < m && y < n {
                diagonal.push(result[x][y]);
                x += 1;
                y += 1;
            }
            diagonal.sort();
            x = d;
            y = 0;
            let mut idx = 0;
            while x < m && y < n {
                result[x][y] = diagonal[idx];
                idx += 1;
                x += 1;
                y += 1;
            }
        }
        
        result

    }
}
