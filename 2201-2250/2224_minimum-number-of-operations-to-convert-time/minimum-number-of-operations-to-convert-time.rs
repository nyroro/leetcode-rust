
impl Solution {
    pub fn convert_time(current: String, correct: String) -> i32 {
        // 解析时间字符串，将其转换为分钟表示的整数值

        let current_minutes: i32 = Solution::parse_time(&current);
        let correct_minutes: i32 = Solution::parse_time(&correct);
        
        // 计算从当前时间到目标时间的分钟差值

        let mut diff: i32 = correct_minutes - current_minutes;
        if diff < 0 {
            diff += 24 * 60;  // 如果目标时间在当前时间之前，加上一天的分钟数

        }
        
        // 根据分钟差值，计算需要的操作次数

        let mut operations: i32 = 0;
        while diff > 0 {
            if diff >= 60 {
                operations += 1;
                diff -= 60;
            } else if diff >= 15 {
                operations += 1;
                diff -= 15;
            } else if diff >= 5 {
                operations += 1;
                diff -= 5;
            } else {
                operations += 1;
                diff -= 1;
            }
        }
        
        operations

    }
    
    // 辅助函数：将时间字符串转换为分钟表示的整数值

    fn parse_time(time: &String) -> i32 {
        let parts: Vec<&str> = time.split(":").collect();
        let hours: i32 = parts[0].parse().unwrap();
        let minutes: i32 = parts[1].parse().unwrap();
        hours * 60 + minutes

    }
}
