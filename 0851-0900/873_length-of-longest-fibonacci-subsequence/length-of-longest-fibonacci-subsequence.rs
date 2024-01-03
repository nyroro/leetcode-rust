
impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut dp = vec![vec![0; n]; n];
        let mut max_len = 0;

        for i in 2..n {
            let (mut left, mut right) = (0, i - 1);
            while left < right {
                let sum = arr[left] + arr[right];
                if sum < arr[i] {
                    left += 1;
                } else if sum > arr[i] {
                    right -= 1;
                } else {
                    dp[right][i] = dp[left][right] + 1;
                    max_len = max_len.max(dp[right][i] + 2);
                    left += 1;
                    right -= 1;
                }
            }
        }

        if max_len >= 3 {
            max_len

        } else {
            0

        }
    }
}
