
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        // 创建HashMap来记录每个用户的活跃分钟数

        let mut uam_map: HashMap<i32, HashSet<i32>> = HashMap::new();
        
        // 遍历logs数组，计算每个用户的活跃分钟数

        for log in logs {
            let user_id = log[0];
            let timestamp = log[1];
            
            // 将用户ID作为键，将时间戳添加到对应的值（HashSet）中

            let user_set = uam_map.entry(user_id).or_insert(HashSet::new());
            user_set.insert(timestamp);
        }
        
        // 创建大小为k的一维数组answer，并初始化为0

        let mut answer = vec![0; k as usize];
        
        // 遍历HashMap中的值，统计每个用户的活跃分钟数

        for user_set in uam_map.values() {
            let uam = user_set.len() as i32;
            answer[uam as usize - 1] += 1;
        }
        
        answer

    }
}
