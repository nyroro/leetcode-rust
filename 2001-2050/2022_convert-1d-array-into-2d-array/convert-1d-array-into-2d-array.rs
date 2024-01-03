
impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        let len = original.len();
        if len != (m * n) as usize {
            return vec![]; // 返回空的二维数组

        }
        
        let mut result = vec![vec![0; n as usize]; m as usize];
        let mut row = 0;
        let mut col = 0;
        
        for num in original {
            result[row][col] = num;
            col += 1;
            if col == n as usize {
                col = 0;
                row += 1;
            }
        }
        
        result

    }
}
