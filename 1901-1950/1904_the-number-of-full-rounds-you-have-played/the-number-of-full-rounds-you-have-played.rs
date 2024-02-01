
impl Solution {
    pub fn number_of_rounds(login_time: String, logout_time: String) -> i32 {
        // 辅助函数：将时间字符串转换为分钟表示

        fn to_minutes(time: &str) -> i32 {
            let parts: Vec<&str> = time.split(':').collect();
            let hours: i32 = parts[0].parse().unwrap();
            let minutes: i32 = parts[1].parse().unwrap();
            hours * 60 + minutes

        }

        // 将登录时间和登出时间转换为分钟表示

        let login_minutes = to_minutes(&login_time);
        let logout_minutes = to_minutes(&logout_time);

        // 计算登录时间和登出时间所在的完整比赛轮次

        let start_round = (login_minutes + 14) / 15;
        let end_round = logout_minutes / 15;

        // 返回完整比赛轮次的差值

        (end_round - start_round).max(0)
    }
}
