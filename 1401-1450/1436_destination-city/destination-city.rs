
use std::collections::HashSet;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut start_cities = HashSet::new();

        // 遍历路径数组，将所有起始城市添加到哈希集合中

        for path in &paths {
            start_cities.insert(&path[0]);
        }

        // 再次遍历路径数组，对于每个目的地城市，检查它是否存在于哈希集合中

        for path in &paths {
            if !start_cities.contains(&path[1]) {
                return path[1].to_string();  // 如果不存在，说明它是终点城市，返回该城市

            }
        }

        String::new()  // 如果没有终点城市，返回空字符串

    }
}
