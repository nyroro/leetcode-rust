


impl Solution {
    pub fn video_stitching(clips: Vec<Vec<i32>>, time: i32) -> i32 {
        // 创建一个长度为 time+1 的数组，初始值为 time+1

        let mut dp = vec![time + 1; (time + 1) as usize];
        // 将 dp[0] 初始化为 0

        dp[0] = 0;

        // 遍历时间点

        for i in 1..=time {
            // 遍历视频片段

            for clip in &clips {
                // 如果视频片段的开始时间小于当前时间点 i，并且视频片段的结束时间大于等于当前时间点 i

                if clip[0] <= i && i <= clip[1] {
                    // 更新 dp[i] 为 dp[i] 和 dp[视频片段的开始时间] + 1 之间的较小值

                    dp[i as usize] = dp[i as usize].min(dp[clip[0] as usize] + 1);
                }
            }
        }

        // 如果 dp[time] 的值大于 time，则返回 -1，否则返回 dp[time]
        if dp[time as usize] > time {
            -1

        } else {
            dp[time as usize]
        }
    }
}
