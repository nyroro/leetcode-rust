
impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        for (i, &range) in ranges.iter().enumerate() {
            let left = (i as i32 - range).max(0) as usize;
            let right = (i as i32 + range).min(n as i32) as usize;
            dp[left..=right].iter_mut().for_each(|x| *x = (*x).max(right as i32));
        }

        let (mut count, mut end, mut farthest) = (0, 0, 0);
        for i in 0..n {
            farthest = farthest.max(dp[i]);
            if i == end {
                if farthest <= i as i32 {
                    return -1;
                }
                end = farthest as usize;
                count += 1;
            }
        }
        count

    }
}
