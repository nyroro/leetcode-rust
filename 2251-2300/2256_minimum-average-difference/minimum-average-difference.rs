


impl Solution {
    pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut pre = vec![nums[0] as i64; n];
        let mut post = vec![nums[n - 1] as i64; n];

        for i in 1..n {
            pre[i] = nums[i] as i64 + pre[i - 1];
        }

        for i in (0..n - 1).rev() {
            post[i] = nums[i] as i64 + post[i + 1];
        }

        let mut m = i64::MAX;
        let mut f = n - 1;

        for i in 0..n {
            let x = pre[i] / ((i + 1) as i64);
            let y = if i == n - 1 {
                0

            } else {
                post[i + 1] / ((n - i - 1) as i64)
            };

            if m > (x - y).abs() {
                m = (x - y).abs();
                f = i;
            }
        }

        f as i32

    }
}
