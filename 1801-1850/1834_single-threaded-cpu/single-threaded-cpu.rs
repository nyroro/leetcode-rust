
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct Task {
    index: i32,
    enqueue_time: i32,
    processing_time: i32,
}

impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        let mut tasks_with_index: Vec<Task> = tasks.iter().enumerate().map(|(i, t)| Task { index: i as i32, enqueue_time: t[0], processing_time: t[1] }).collect();
        tasks_with_index.sort_by_key(|t| t.enqueue_time);

        let mut result = Vec::new();
        let mut time = 0;
        let mut i = 0;
        let mut available_tasks = BinaryHeap::new();

        while !available_tasks.is_empty() || i < tasks_with_index.len() {
            while i < tasks_with_index.len() && tasks_with_index[i].enqueue_time <= time {
                available_tasks.push(Reverse((tasks_with_index[i].processing_time, tasks_with_index[i].index)));
                i += 1;
            }

            if let Some(Reverse((processing_time, index))) = available_tasks.pop() {
                time += processing_time;
                result.push(index);
            } else {
                time = tasks_with_index[i].enqueue_time;
            }
        }

        result

    }
}
