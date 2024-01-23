
impl Solution {
    pub fn possible_to_stamp(grid: Vec<Vec<i32>>, stamp_height: i32, stamp_width: i32) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let mut s = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                s[i][j] = grid[i][j];
                if j > 0 {
                    s[i][j] += s[i][j - 1];
                }
            }
            if i > 0 {
                for j in 0..n {
                    s[i][j] += s[i - 1][j];
                }
            }
        }

        let mut c = vec![0; n];
        let mut w = 0;
        let mut h = 0;

        fn can_cover(x: usize, y: usize, stamp_height: i32, stamp_width: i32, s: &Vec<Vec<i32>>) -> bool {
            if x + stamp_height as usize - 1 >= s.len() || y + stamp_width as usize - 1 >= s[0].len() {
                return false;
            }
            let mut t = s[x + stamp_height as usize - 1][y + stamp_width as usize - 1];
            if x > 0 {
                t -= s[x - 1][y + stamp_width as usize - 1];
            }
            if y > 0 {
                t -= s[x + stamp_height as usize - 1][y - 1];
            }
            if x > 0 && y > 0 {
                t += s[x - 1][y - 1];
            }
            t == 0

        }

        for i in 0..m {
            w = 0;
            h = 0;
            for j in 0..n {
                if grid[i][j] == 0 {
                    if can_cover(i, j, stamp_height, stamp_width, &s) {
                        w = stamp_width - 1;
                        h = stamp_height;
                        c[j] = h;
                        continue;
                    }
                    if w > 0 && h > 0 {
                        w -= 1;
                        c[j] = h;
                        continue;
                    }
                    if c[j] > 0 {
                        continue;
                    }
                    return false;
                }
            }
            for j in 0..n {
                if c[j] > 0 {
                    c[j] -= 1;
                }
            }
        }
        true

    }
}
