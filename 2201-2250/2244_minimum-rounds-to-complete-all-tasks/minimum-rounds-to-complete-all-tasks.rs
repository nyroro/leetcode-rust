
use std::collections::HashMap;

impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let mut task_count: HashMap<i32, i32> = HashMap::new();
        let mut round_count = 0;

        // 统计每个任务的数量

        for task in tasks {
            *task_count.entry(task).or_insert(0) += 1;
        }

        // 遍历哈希表，计算完成每个任务所需的最小轮数

        for (_, count) in task_count {
            if count < 2 {
                return -1;
            }
            round_count += (count + 2) / 3;
        }

        round_count

    }
}
