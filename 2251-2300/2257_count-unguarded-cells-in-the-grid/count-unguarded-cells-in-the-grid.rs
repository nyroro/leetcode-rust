
impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let mut cell = vec![vec![0; n as usize]; m as usize];

        for wall in walls {
            let x = wall[0] as usize;
            let y = wall[1] as usize;
            cell[x][y] = 1;
        }

        for guard in guards {
            let x = guard[0] as usize;
            let y = guard[1] as usize;
            cell[x][y] = 1;
        }

        fn gao(x: usize, y: usize, d: (i32, i32), cell: &mut Vec<Vec<i32>>) {
            let (mut x, mut y) = (x as i32, y as i32);
            let (dx, dy) = d;
            x += dx;
            y += dy;
            while x >= 0 && x < m && y >= 0 && y < n && cell[x as usize][y as usize] != 1 {
                cell[x as usize][y as usize] = 2;
                x += dx;
                y += dy;
            }
        }

        for guard in guards {
            let x = guard[0] as usize;
            let y = guard[1] as usize;
            gao(x, y, (0, 1), &mut cell);
            gao(x, y, (0, -1), &mut cell);
            gao(x, y, (1, 0), &mut cell);
            gao(x, y, (-1, 0), &mut cell);
        }

        let mut ret = 0;
        for i in 0..m as usize {
            for j in 0..n as usize {
                if cell[i][j] == 0 {
                    ret += 1;
                }
            }
        }
        ret

    }
}
