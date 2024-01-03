
impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut heights = vec![vec![0; n]; m];
        
        // 计算每一行的直方图

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 1 {
                    heights[i][j] = if i > 0 { heights[i-1][j] + 1 } else { 1 };
                }
            }
        }
        
        let mut max_area = 0;
        
        // 对每一行的直方图进行排序，并计算最大面积

        for i in 0..m {
            let mut row = heights[i].clone();
            row.sort();
            row.reverse();
            
            for j in 0..n {
                let area = row[j] * (j as i32 + 1);
                max_area = max_area.max(area);
            }
        }
        
        max_area

    }
}
