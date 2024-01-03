
impl Solution {
    pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
        // 创建一个新的数组来存储每一列的最大长度

        let mut ans: Vec<i32> = vec![0; grid[0].len()];

        // 遍历二维数组，计算每一列中数字的长度，并找到最大的长度

        for row in &grid {
            for (i, &num) in row.iter().enumerate() {
                let len = if num >= 0 {
                    num.to_string().len() as i32

                } else {
                    (num.abs().to_string().len() + 1) as i32

                };
                ans[i] = ans[i].max(len);
            }
        }

        // 将这些最大长度存储在新的数组中并返回

        ans

    }
}
