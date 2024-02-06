
use std::collections::VecDeque;

struct BoxInfo {
    status: i32,
    candies: i32,
    keys: Vec<i32>,
    contained_boxes: Vec<i32>,
}

impl Solution {
    pub fn max_candies(status: Vec<i32>, candies: Vec<i32>, keys: Vec<Vec<i32>>, contained_boxes: Vec<Vec<i32>>, initial_boxes: Vec<i32>) -> i32 {
        let n = status.len();
        let mut visited = vec![false; n];
        let mut queue: VecDeque<i32> = VecDeque::new();
        let mut ans = 0;

        // Initialize the queue with initial boxes

        for &box_id in &initial_boxes {
            queue.push_back(box_id);
            visited[box_id as usize] = true;
        }

        while let Some(box_id) = queue.pop_front() {
            if status[box_id as usize] == 1 || keys.iter().any(|k| k.contains(&box_id)) {
                ans += candies[box_id as usize];
                for &contained_box in &contained_boxes[box_id as usize] {
                    queue.push_back(contained_box);
                    visited[contained_box as usize] = true;
                }
            }
        }

        ans

    }
}
