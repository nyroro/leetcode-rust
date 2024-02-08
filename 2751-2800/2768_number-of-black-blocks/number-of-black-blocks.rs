


impl Solution {
    pub fn count_black_blocks(m: i32, n: i32, coordinates: Vec<Vec<i32>>) -> Vec<i64> {
        use std::collections::HashSet;

        let mut vis: HashSet<(i32, i32)> = HashSet::new();
        let mut s: HashSet<(i32, i32)> = HashSet::new();
        for co in &coordinates {
            s.insert((co[0], co[1]));
        }
        let mut ans: Vec<i64> = vec![0; 5];

        for co in &coordinates {
            let x = co[0];
            let y = co[1];
            if x > 0 && y > 0 {
                if !vis.contains(&(x - 1, y - 1)) && !vis.contains(&(x - 1, y)) && !vis.contains(&(x, y - 1)) {
                    let mut cnt = 1;
                    if s.contains(&(x - 1, y - 1)) { cnt += 1; }
                    if s.contains(&(x - 1, y)) { cnt += 1; }
                    if s.contains(&(x, y - 1)) { cnt += 1; }
                    ans[cnt as usize] += 1;
                }
            }
            if x + 1 < m && y > 0 {
                if !vis.contains(&(x + 1, y - 1)) && !vis.contains(&(x + 1, y)) && !vis.contains(&(x, y - 1)) {
                    let mut cnt = 1;
                    if s.contains(&(x + 1, y - 1)) { cnt += 1; }
                    if s.contains(&(x + 1, y)) { cnt += 1; }
                    if s.contains(&(x, y - 1)) { cnt += 1; }
                    ans[cnt as usize] += 1;
                }
            }
            if x > 0 && y + 1 < n {
                if !vis.contains(&(x - 1, y + 1)) && !vis.contains(&(x - 1, y)) && !vis.contains(&(x, y + 1)) {
                    let mut cnt = 1;
                    if s.contains(&(x - 1, y + 1)) { cnt += 1; }
                    if s.contains(&(x - 1, y)) { cnt += 1; }
                    if s.contains(&(x, y + 1)) { cnt += 1; }
                    ans[cnt as usize] += 1;
                }
            }
            if x + 1 < m && y + 1 < n {
                if !vis.contains(&(x + 1, y + 1)) && !vis.contains(&(x + 1, y)) && !vis.contains(&(x, y + 1)) {
                    let mut cnt = 1;
                    if s.contains(&(x + 1, y + 1)) { cnt += 1; }
                    if s.contains(&(x + 1, y)) { cnt += 1; }
                    if s.contains(&(x, y + 1)) { cnt += 1; }
                    ans[cnt as usize] += 1;
                }
            }
            vis.insert((x, y));
        }

        let tot = ans[1] + ans[2] + ans[3] + ans[4];
        let n = n as i64 - 1;
        let m = m as i64 - 1;
        let z = n * m;
        ans[0] = z - tot;
        ans

    }
}
