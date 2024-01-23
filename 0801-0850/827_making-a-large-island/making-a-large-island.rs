


impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        // 创建一个二维数组来标记岛屿

        let mut island = vec![vec![0; grid[0].len()]; grid.len()];
        let mut cnt = std::collections::HashMap::new();
        let mut mark = 1;
        let height = grid.len();
        let width = grid[0].len();

        // 定义DFS函数

        fn dfs(x: usize, y: usize, mark: i32, grid: &Vec<Vec<i32>>, island: &mut Vec<Vec<i32>>, cnt: &mut std::collections::HashMap<i32, i32>) {
            if x >= 0 && x < grid.len() && y >= 0 && y < grid[0].len() && grid[x][y] == 1 && island[x][y] == 0 {
                island[x][y] = mark;
                *cnt.entry(mark).or_insert(0) += 1;
                dfs(x + 1, y, mark, grid, island, cnt);
                dfs(x - 1, y, mark, grid, island, cnt);
                dfs(x, y + 1, mark, grid, island, cnt);
                dfs(x, y - 1, mark, grid, island, cnt);
            }
        }

        // 遍历grid，标记岛屿并计算大小

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 && island[i][j] == 0 {
                    cnt.insert(mark, 0);
                    dfs(i, j, mark, &grid, &mut island, &mut cnt);
                    mark += 1;
                }
            }
        }

        let mut ret = 0;

        // 计算最大岛屿大小

        for &value in cnt.values() {
            ret = ret.max(value);
        }

        // 尝试将0变为1，并计算新的岛屿大小

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 0 {
                    let mut s = std::collections::HashSet::new();
                    if i > 0 {
                        s.insert(island[i - 1][j]);
                    }
                    if i < height - 1 {
                        s.insert(island[i + 1][j]);
                    }
                    if j > 0 {
                        s.insert(island[i][j - 1]);
                    }
                    if j < width - 1 {
                        s.insert(island[i][j + 1]);
                    }
                    let mut t = 1;
                    for &k in &s {
                        t += cnt.get(&k).unwrap_or(&0);
                    }
                    ret = ret.max(t);
                }
            }
        }

        ret

    }
}
