
impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        // 将时间点转换为分钟表示，并存储在一个数组中

        let mut minutes: Vec<i32> = time_points

            .iter()
            .map(|time| {
                let parts: Vec<&str> = time.split(':').collect();
                let hours = parts[0].parse::<i32>().unwrap();
                let mins = parts[1].parse::<i32>().unwrap();
                hours * 60 + mins

            })
            .collect();

        // 对分钟数组进行排序

        minutes.sort();

        // 计算最小分钟差

        let mut min_diff = i32::MAX;
        for i in 1..minutes.len() {
            let diff = minutes[i] - minutes[i - 1];
            min_diff = min_diff.min(diff);
        }

        // 考虑首尾时间点之间的差值

        let first = minutes[0];
        let last = minutes[minutes.len() - 1];
        let circular_diff = 24 * 60 - last + first;
        min_diff = min_diff.min(circular_diff);

        min_diff

    }
}
