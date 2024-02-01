
impl Solution {
    pub fn min_processing_time(processor_time: Vec<i32>, tasks: Vec<i32>) -> i32 {
        let mut tasks = tasks;
        tasks.sort_by(|a, b| b.cmp(a)); // Sort tasks in descending order

        let mut processor_time = processor_time;
        processor_time.sort(); // Sort processor_time in ascending order

        let mut left = 0;
        let mut right = tasks.len() - 1;
        while left < right {
            tasks.swap(left, right);
            left += 1;
            right -= 1;
        }
        let mut l = 0;
        let mut r = tasks.len() - 1;
        let mut res = 0;
        while l <= r {
            res = res.max(tasks[l] + processor_time[l / 4]);
            l += 1;
        }
        res

    }
}
