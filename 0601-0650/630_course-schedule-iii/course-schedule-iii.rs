
use std::collections::BinaryHeap;

impl Solution {
    pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
        let mut courses = courses;
        courses.sort_by_key(|c| c[1]); // Sort courses by end date

        let mut time = 0;
        let mut heap = BinaryHeap::new();
        
        for course in courses {
            let duration = course[0] as usize;
            let end = course[1] as usize;
            time += duration;
            heap.push(duration as i32);
            
            if time > end {
                if let Some(max_duration) = heap.pop() {
                    time -= max_duration as usize;
                }
            }
        }
        
        heap.len() as i32

    }
}
