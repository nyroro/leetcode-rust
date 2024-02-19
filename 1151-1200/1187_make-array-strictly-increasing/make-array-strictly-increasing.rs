


impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut arr2 = arr2;
        arr2.sort();
        arr2.dedup();

        let n = arr1.len();
        let m = arr2.len();
        let mut dp = vec![vec![2 * n; m + 1]; n + 1];
        dp[0][0] = 0;

        for j in 1..=m {
            dp[0][j] = 2 * n;
        }

        for i in 1..=n {
            dp[i][0] = 2 * n;
            if i == 1 || arr1[i - 1] > arr1[i - 2] {
                dp[i][0] = dp[i - 1][0];
            }
            let mut cur_min = 2 * n;
            for j in 1..=m {
                if i == 1 || arr1[i - 1] > arr2[j - 1] {
                    dp[i][0] = dp[i][0].min(dp[i - 1][j]);
                }
                dp[i][j] = cur_min + 1;
                if i == 1 || arr2[j - 1] > arr1[i - 2] {
                    dp[i][j] = dp[i][j].min(dp[i - 1][0] + 1);
                }
                cur_min = cur_min.min(dp[i - 1][j]);
            }
        }

        let mut ans = 2 * n;
        for j in 0..=m {
            ans = ans.min(dp[n][j]);
        }

        if ans > n {
            -1

        } else {
            ans as i32  // Explicit conversion to i32

        }
    }
}
