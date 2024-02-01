
impl Solution {
    pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
        // 将时间转换为分钟表示

        let start_time1: i32 = event1[0].split(':').collect::<Vec<&str>>()[0].parse().unwrap() * 60

            + event1[0].split(':').collect::<Vec<&str>>()[1].parse().unwrap();
        let end_time1: i32 = event1[1].split(':').collect::<Vec<&str>>()[0].parse().unwrap() * 60

            + event1[1].split(':').collect::<Vec<&str>>()[1].parse().unwrap();
        let start_time2: i32 = event2[0].split(':').collect::<Vec<&str>>()[0].parse().unwrap() * 60

            + event2[0].split(':').collect::<Vec<&str>>()[1].parse().unwrap();
        let end_time2: i32 = event2[1].split(':').collect::<Vec<&str>>()[0].parse().unwrap() * 60

            + event2[1].split(':').collect::<Vec<&str>>()[1].parse().unwrap();
        
        // 判断是否存在重叠部分

        if start_time1 <= end_time2 && start_time2 <= end_time1 {
            return true;
        }
        
        false

    }
}
