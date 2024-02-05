
use std::collections::HashSet;



impl Solution {
    pub fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i64 {
        let mut area = 0i64;
        let mut rectangles = rectangles;
        rectangles.sort_by_key(|r| r[1]);

        let mut x_grid = HashSet::new();
        for rect in &rectangles {
            x_grid.insert(rect[0] as i64);
            x_grid.insert(rect[2] as i64);
        }

        let mut x_grid: Vec<i64> = x_grid.into_iter().collect();
        x_grid.sort();

        for i in 0..x_grid.len() - 1 {
            let mut y_min = 0i64;
            let dx = x_grid[i + 1] - x_grid[i];
            for rect in &rectangles {
                let (x1, y1, x2, y2) = (rect[0] as i64, rect[1] as i64, rect[2] as i64, rect[3] as i64);
                if x1 <= x_grid[i] && x2 >= x_grid[i + 1] {
                    if y1 >= y_min {
                        area += dx * (y2 - y1) % (10i64.pow(9) + 7);
                        y_min = y2;
                    } else if y2 > y_min {
                        area += dx * (y2 - y_min) % (10i64.pow(9) + 7);
                        y_min = y2;
                    }
                }
            }
        }

        area % (10i64.pow(9) + 7)
    }
}
