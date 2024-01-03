
impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        // 创建一个二维数组grid，初始值为1

        let mut grid = vec![vec![1; n as usize]; n as usize];
        
        // 将mines中的0位置标记为0

        for mine in mines {
            let row = mine[0] as usize;
            let col = mine[1] as usize;
            grid[row][col] = 0;
        }
        
        // 创建四个二维数组，分别表示上、下、左、右四个方向的连续1的个数

        let mut left = vec![vec![0; n as usize]; n as usize];
        let mut right = vec![vec![0; n as usize]; n as usize];
        let mut up = vec![vec![0; n as usize]; n as usize];
        let mut down = vec![vec![0; n as usize]; n as usize];
        
        // 计算左边和上边的连续1的个数

        for i in 0..n as usize {
            for j in 0..n as usize {
                if grid[i][j] == 1 {
                    if j > 0 {
                        left[i][j] = left[i][j - 1] + 1;
                    } else {
                        left[i][j] = 1;
                    }
                    if i > 0 {
                        up[i][j] = up[i - 1][j] + 1;
                    } else {
                        up[i][j] = 1;
                    }
                }
            }
        }
        
        // 计算右边和下边的连续1的个数

        for i in (0..n as usize).rev() {
            for j in (0..n as usize).rev() {
                if grid[i][j] == 1 {
                    if j < n as usize - 1 {
                        right[i][j] = right[i][j + 1] + 1;
                    } else {
                        right[i][j] = 1;
                    }
                    if i < n as usize - 1 {
                        down[i][j] = down[i + 1][j] + 1;
                    } else {
                        down[i][j] = 1;
                    }
                }
            }
        }
        
        // 找到最大的加号符号的阶数

        let mut max_order = 0;
        for i in 0..n as usize {
            for j in 0..n as usize {
                let order = left[i][j].min(right[i][j]).min(up[i][j]).min(down[i][j]);
                max_order = max_order.max(order);
            }
        }
        
        max_order

    }
}
