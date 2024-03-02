
impl Solution {
    pub fn max_task_assign(tasks: Vec<i32>, workers: Vec<i32>, pills: i32, strength: i32) -> i32 {
        let mut tasks = tasks;
        let mut workers = workers;
        tasks.sort();
        workers.sort();

        fn is_possible(num_task: usize, tasks: &Vec<i32>, workers: &mut Vec<i32>, pills: i32, strength: i32) -> bool {
            let mut pills = pills;
            let mut workers = workers.split_off(workers.len() - num_task);

            for i in (0..num_task).rev() {
                if let Some(worker) = workers.last() {
                    if *worker < tasks[i] {
                        if pills == 0 {
                            return false;
                        }
                        if let Some(idx) = workers.iter().position(|&w| w >= tasks[i] - strength) {
                            workers.remove(idx);
                            pills -= 1;
                        } else {
                            return false;
                        }
                    } else {
                        workers.pop();
                    }
                } else {
                    return false;
                }
            }
            true

        }

        let mut low = 0;
        let mut high = std::cmp::min(tasks.len(), workers.len());
        let mut ans = 0;

        while low <= high {
            let mid = (low + high) / 2;
            if is_possible(mid, &tasks, &mut workers.clone(), pills, strength) {
                ans = mid as i32;
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        ans

    }
}
