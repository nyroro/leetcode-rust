


impl Solution {
    pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
        let mut left: Vec<[usize; 2]> = Vec::new();
        let mut right: Vec<[usize; 2]> = Vec::new();

        for i in 0..3 {
            for j in 0..3 {
                if grid[i][j] == 0 {
                    left.push([i, j]);
                } else {
                    for k in 1..grid[i][j] {
                        right.push([i, j]);
                    }
                }
            }
        }

        let cal = |a: [usize; 2], b: [usize; 2]| -> i32 {
            ((a[0] as i32 - b[0] as i32).abs() + (a[1] as i32 - b[1] as i32).abs()) as i32

        };

        let n = left.len();
        let mut f: Vec<i32> = vec![1 << 30; 1 << n];
        f[0] = 0;

        for i in 1..1 << n {
            let mut k = 0;
            for j in 0..n {
                if (i >> j) & 1 == 1 {
                    k += 1;
                }
            }
            for j in 0..n {
                if (i >> j) & 1 == 1 {
                    f[i] = f[i].min(f[i ^ (1 << j)] + cal(left[k - 1], right[j]));
                }
            }
        }

        f[(1 << n) - 1]
    }
}
