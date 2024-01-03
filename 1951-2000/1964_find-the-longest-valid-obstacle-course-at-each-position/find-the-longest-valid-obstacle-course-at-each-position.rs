
impl Solution {
    pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
        let n = obstacles.len();
        let mut dp = vec![0; n];
        let mut tails = vec![0; n];
        let mut len = 0;

        for i in 0..n {
            let mut left = 0;
            let mut right = len;
            while left < right {
                let mid = left + (right - left) / 2;
                if tails[mid] <= obstacles[i] {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            dp[i] = left as i32 + 1;
            tails[left] = obstacles[i];
            if left == len {
                len += 1;
            }
        }

        dp

    }
}
