
impl Solution {
    pub fn count_tested_devices(mut battery_percentages: Vec<i32>) -> i32 {
        let mut tested_devices = 0;
        let n = battery_percentages.len();

        for i in 0..n {
            if battery_percentages[i] > 0 {
                tested_devices += 1;
                for j in (i + 1)..n {
                    if battery_percentages[j] > 0 {
                        battery_percentages[j] = std::cmp::max(0, battery_percentages[j] - 1);
                    }
                }
            }
        }

        tested_devices

    }
}
