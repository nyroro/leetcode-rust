
impl Solution {
    pub fn closet_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let mut min_distance = std::i32::MAX;
        let n = words.len() as i32;
        let mut target_indices: Vec<i32> = Vec::new();

        // 找到目标字符串的所有索引位置

        for (i, word) in words.iter().enumerate() {
            if word == &target {
                target_indices.push(i as i32);
            }
        }

        // 计算最短距离

        for &index in &target_indices {
            let distance = (index - start_index).abs().min(n - (index - start_index).abs());
            min_distance = min_distance.min(distance);
        }

        // 返回最短距离，如果目标字符串不存在于 words 中，则返回 -1

        if min_distance == std::i32::MAX {
            -1

        } else {
            min_distance

        }
    }
}
