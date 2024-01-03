
impl Solution {
    pub fn find_delayed_arrival_time(arrival_time: i32, delayed_time: i32) -> i32 {
        // 计算延迟到达时间

        let delayed_arrival_time = (arrival_time + delayed_time) % 24;
        // 返回结果

        delayed_arrival_time

    }
}
