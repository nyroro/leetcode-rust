
impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let m = height_map.len();
        let n = height_map[0].len();
        let mut volume = 0;
        let mut visited = vec![vec![false; n]; m];
        let mut min_heap = std::collections::BinaryHeap::new();
        
        // 将边界上的单元格加入最小堆

        for i in 0..m {
            visited[i][0] = true;
            visited[i][n - 1] = true;
            min_heap.push(Cell::new(i, 0, height_map[i][0]));
            min_heap.push(Cell::new(i, n - 1, height_map[i][n - 1]));
        }
        for j in 0..n {
            visited[0][j] = true;
            visited[m - 1][j] = true;
            min_heap.push(Cell::new(0, j, height_map[0][j]));
            min_heap.push(Cell::new(m - 1, j, height_map[m - 1][j]));
        }
        
        // 使用最小堆进行广度优先搜索

        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        while let Some(cell) = min_heap.pop() {
            for dir in &directions {
                let ni = cell.row + dir.0;
                let nj = cell.col + dir.1;
                if ni >= 0 && ni < m as i32 && nj >= 0 && nj < n as i32 && !visited[ni as usize][nj as usize] {
                    visited[ni as usize][nj as usize] = true;
                    let height = height_map[ni as usize][nj as usize];
                    volume += (cell.height - height).max(0);
                    min_heap.push(Cell::new(ni as usize, nj as usize, height.max(cell.height)));
                }
            }
        }
        
        volume

    }
}

// 定义一个表示单元格的结构体

#[derive(Eq, PartialEq)]
struct Cell {
    row: usize,
    col: usize,
    height: i32,
}

impl Cell {
    fn new(row: usize, col: usize, height: i32) -> Self {
        Cell { row, col, height }
    }
}

// 测试代码

fn main() {
    let height_map = vec![
        vec![1, 4, 3, 1, 3, 2],
        vec![3, 2, 1, 3, 2, 4],
        vec![2, 3, 3, 2, 3, 1],
    ];
    let volume = Solution::trap_rain_water(height_map);
    println!("Volume of trapped water: {}", volume);
}
