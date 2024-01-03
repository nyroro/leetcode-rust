
impl Solution {
    pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
        let mut count = 0; // 初始化计数器为0


        for &hour in hours.iter() {
            if hour >= target {
                count += 1; // 工作时间大于等于目标值，计数器加一

            }
        }

        count // 返回计数器的值作为结果

    }
}
