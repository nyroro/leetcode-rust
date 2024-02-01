
use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        // 构建图的邻接表表示

        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut in_degree: Vec<i32> = vec![0; n as usize];
        for relation in &relations {
            let prev = relation[0];
            let next = relation[1];
            graph.entry(prev).or_insert(Vec::new()).push(next);
            in_degree[next as usize - 1] += 1;
        }
        
        // 计算每门课程的完成时间

        let mut completion_time: Vec<i32> = vec![0; n as usize];
        for i in 0..n as usize {
            completion_time[i] = time[i];
        }
        
        // 拓扑排序

        let mut queue: VecDeque<i32> = VecDeque::new();
        for i in 0..n as usize {
            if in_degree[i] == 0 {
                queue.push_back(i as i32 + 1);
            }
        }
        
        while let Some(course) = queue.pop_front() {
            if let Some(next_courses) = graph.get(&course) {
                for next_course in next_courses {
                    let next_course = *next_course;
                    in_degree[next_course as usize - 1] -= 1;
                    completion_time[next_course as usize - 1] = completion_time[next_course as usize - 1].max(completion_time[course as usize - 1] + time[next_course as usize - 1]);
                    if in_degree[next_course as usize - 1] == 0 {
                        queue.push_back(next_course);
                    }
                }
            }
        }
        
        // 返回最少时间

        *completion_time.iter().max().unwrap()
    }
}
