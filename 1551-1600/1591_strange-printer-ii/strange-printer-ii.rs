
use std::collections::HashMap;

impl Solution {
    pub fn is_printable(target_grid: Vec<Vec<i32>>) -> bool {
        let m = target_grid.len();
        let n = target_grid[0].len();
        let mut colors: HashMap<i32, (usize, usize, usize, usize)> = HashMap::new();
        let mut visited: HashMap<i32, bool> = HashMap::new();

        // 记录每种颜色的矩形的边界

        for i in 0..m {
            for j in 0..n {
                let color = target_grid[i][j];
                let (left, right, top, bottom) = colors.entry(color).or_insert((m, 0, n, 0));
                *left = (*left).min(i);
                *right = (*right).max(i);
                *top = (*top).min(j);
                *bottom = (*bottom).max(j);
            }
        }

        // 深度优先搜索

        fn dfs(color: i32, colors: &HashMap<i32, (usize, usize, usize, usize)>, visited: &mut HashMap<i32, bool>, target_grid: &Vec<Vec<i32>>) -> bool {
            if let Some(&visited_color) = visited.get(&color) {
                return visited_color;
            }
            visited.insert(color, false);

            let (left, right, top, bottom) = colors.get(&color).unwrap();
            for i in *left..=*right {
                for j in *top..=*bottom {
                    let neighbor_color = target_grid[i][j];
                    if neighbor_color != color {
                        if !dfs(neighbor_color, colors, visited, target_grid) {
                            return false;
                        }
                    }
                }
            }

            visited.insert(color, true);
            true

        }

        // 遍历每种颜色节点

        for color in colors.keys() {
            if !dfs(*color, &colors, &mut visited, &target_grid) {
                return false;
            }
        }

        true

    }
}
