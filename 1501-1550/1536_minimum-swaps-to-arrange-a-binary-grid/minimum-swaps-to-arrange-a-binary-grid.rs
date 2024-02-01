
impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        // Check the validity of the input grid

        let n = grid.len();
        if n != grid[0].len() || n < 1 || n > 200 {
            return -1;
        }

        let mut swaps: i32 = 0;  // Fix the type of swaps to i32

        let mut zeros_count = vec![0; n];

        // Calculate the count of trailing zeros for each row

        for i in 0..n {
            let mut count = 0;
            for j in (0..n).rev() {
                if grid[i][j] == 0 {
                    count += 1;
                } else {
                    break;
                }
            }
            zeros_count[i] = count;
        }

        // Greedy algorithm to find the minimum swaps

        for i in 0..n {
            let mut found = false;
            for j in i..n {
                if zeros_count[j] >= n - i - 1 {
                    found = true;
                    for k in (i..j).rev() {
                        zeros_count.swap(k, k + 1);
                    }
                    swaps += (j - i) as i32;  // Convert the result of (j - i) to i32

                    break;
                }
            }
            if !found {
                return -1;
            }
        }

        swaps

    }
}
