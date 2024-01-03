
use std::collections::HashMap;

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        // 构建员工的层级关系

        let mut hierarchy: HashMap<i32, Vec<i32>> = HashMap::new();
        for (i, &m) in manager.iter().enumerate() {
            hierarchy.entry(m).or_insert(Vec::new()).push(i as i32);
        }
        
        // 递归计算通知时间

        fn calculate_time(id: i32, hierarchy: &HashMap<i32, Vec<i32>>, inform_time: &Vec<i32>) -> i32 {
            let subordinates = hierarchy.get(&id);
            let mut time = inform_time[id as usize];
            if let Some(subordinates) = subordinates {
                for &subordinate in subordinates {
                    time = time.max(inform_time[id as usize] + calculate_time(subordinate, hierarchy, inform_time));
                }
            }
            time

        }
        
        calculate_time(head_id, &hierarchy, &inform_time)
    }
}
