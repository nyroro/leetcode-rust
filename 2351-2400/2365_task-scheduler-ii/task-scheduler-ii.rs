
// Define the Solution struct



impl Solution {
    // Define the task_scheduler_ii function

    pub fn task_scheduler_ii(tasks: Vec<i32>, space: i32) -> i64 {
        // Create a HashMap to store the count of each task

        let mut count_dict: std::collections::HashMap<i32, i64> = std::collections::HashMap::new();
        // Initialize total_days to 0

        let mut total_days: i64 = 0;

        // Iterate through the tasks

        for &task in tasks.iter() {
            // If the task is not in the count_dict, set its count to negative infinity

            if !count_dict.contains_key(&task) {
                count_dict.insert(task, std::i64::MIN);
            }
            // Update total_days based on the maximum of (total_days + 1) and (count_dict[task] + space + 1)
            total_days = std::cmp::max(total_days + 1, count_dict[&task] + space as i64 + 1);
            // Update the count_dict with the current total_days

            count_dict.insert(task, total_days);
        }

        // Return the total_days as the result

        total_days

    }
}
