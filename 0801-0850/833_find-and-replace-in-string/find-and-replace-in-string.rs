
struct Replacement {
    index: i32,
    source: String,
    target: String,
}

impl Solution {
    pub fn find_replace_string(s: String, indices: Vec<i32>, sources: Vec<String>, targets: Vec<String>) -> String {
        let mut replacements: Vec<Replacement> = Vec::new();
        
        // 创建替换操作信息

        for i in 0..indices.len() {
            let replacement = Replacement {
                index: indices[i],
                source: sources[i].clone(),
                target: targets[i].clone(),
            };
            replacements.push(replacement);
        }
        
        // 按照索引对替换操作进行排序

        replacements.sort_by_key(|r| r.index);
        
        let mut result = String::from(s);
        let mut offset = 0;  // 用于处理替换操作后索引的偏移量
        
        // 执行替换操作

        for replacement in replacements {
            let index = (replacement.index + offset) as usize;
            if result[index..].starts_with(&replacement.source) {
                result = format!("{}{}{}", &result[..index], &replacement.target, &result[index + replacement.source.len()..]);
                offset += replacement.target.len() as i32 - replacement.source.len() as i32;  // 更新偏移量

            }
        }
        
        result

    }
}
