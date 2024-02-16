
impl Solution {
    pub fn max_trailing_zeros(grid: Vec<Vec<i32>>) -> i32 {
        fn find(n: i32) -> (i32, i32) {
            let (mut count2, mut count5) = (0, 0);
            let mut n = n;
            while n % 2 == 0 {
                n /= 2;
                count2 += 1;
            }
            while n % 5 == 0 {
                n /= 5;
                count5 += 1;
            }
            (count2, count5)
        }

        let n = grid.len();
        let m = grid[0].len();
        let mut nums = vec![vec![(0, 0); m]; n];
        let mut top = vec![vec![(0, 0); m]; n];
        let mut left = vec![vec![(0, 0); m]; n];
        let mut right = vec![vec![(0, 0); m]; n];
        let mut bottom = vec![vec![(0, 0); m]; n];

        for i in 0..n {
            for j in 0..m {
                let (count2, count5) = find(grid[i][j]);
                nums[i][j] = (count2, count5);
            }
        }

        top = nums.clone();
        left = nums.clone();
        right = nums.clone();
        bottom = nums.clone();

        for i in 0..n {
            for j in 0..m {
                if i > 0 {
                    top[i][j].0 += top[i - 1][j].0;
                    top[i][j].1 += top[i - 1][j].1;
                }
                if j > 0 {
                    left[i][j].0 += left[i][j - 1].0;
                    left[i][j].1 += left[i][j - 1].1;
                }
            }
        }

        for i in 0..n {
            for j in (0..m).rev() {
                if j < m - 1 {
                    right[i][j].0 += right[i][j + 1].0;
                    right[i][j].1 += right[i][j + 1].1;
                }
            }
        }

        for i in (0..n).rev() {
            for j in 0..m {
                if i < n - 1 {
                    bottom[i][j].0 += bottom[i + 1][j].0;
                    bottom[i][j].1 += bottom[i + 1][j].1;
                }
            }
        }

        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                let (curr2, curr5) = nums[i][j];
                let (top2, top5) = top[i][j];
                let (right2, right5) = right[i][j];
                let (bottom2, bottom5) = bottom[i][j];
                let (left2, left5) = left[i][j];

                ans = ans.max(
                    (top2 + right2 - curr2).min(top5 + right5 - curr5),
                );
                ans = ans.max(
                    (top2 + left2 - curr2).min(top5 + left5 - curr5),
                );
                ans = ans.max(
                    (bottom2 + right2 - curr2).min(bottom5 + right5 - curr5),
                );
                ans = ans.max(
                    (bottom2 + left2 - curr2).min(bottom5 + left5 - curr5),
                );
            }
        }
        ans

    }
}
