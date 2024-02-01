
impl Solution {
    pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
        // 将时间转换为分钟表示

        let start_time1: i32 = Self::convert_to_minutes(&event1[0]);
        let end_time1: i32 = Self::convert_to_minutes(&event1[1]);
        let start_time2: i32 = Self::convert_to_minutes(&event2[0]);
        let end_time2: i32 = Self::convert_to_minutes(&event2[1]);
        
        // 判断是否存在重叠部分

        if start_time1 <= end_time2 && start_time2 <= end_time1 {
            return true;
        }
        
        false

    }
    
    fn convert_to_minutes(time: &String) -> i32 {
        let parts: Vec<&str> = time.split(':').collect();
        let hours: i32 = parts[0].parse().unwrap();
        let minutes: i32 = parts[1].parse().unwrap();
        hours * 60 + minutes

    }
}
