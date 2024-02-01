
use std::collections::HashMap;

impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let mut char_indices = HashMap::new();
        
        // 遍历字符串s，记录每个字母的出现位置

        for (i, c) in s.chars().enumerate() {
            let entry = char_indices.entry(c.to_ascii_lowercase()).or_insert(Vec::new());
            entry.push(i as i32);
        }
        
        // 遍历distance数组，检查每个字母的出现位置之间的距离

        for (c, d) in char_indices.iter() {
            let d_len = d.len();
            if d_len != 2 {
                return false;
            }
            let dist = (d[1] - d[0]).abs() - 1;
            let idx = (c.to_ascii_lowercase() as u8 - b'a') as usize;
            if dist != distance[idx] {
                return false;
            }
        }
        
        return true;
    }
}
