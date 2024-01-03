
impl Solution {
    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        // 构建一个映射表，将底部的两个块映射到允许的顶部块

        let mut mapping: Vec<Vec<Vec<char>>> = vec![vec![vec![]; 7]; 7];
        for s in allowed {
            let chars: Vec<char> = s.chars().collect();
            let bottom1 = chars[0] as usize - 'A' as usize;
            let bottom2 = chars[1] as usize - 'A' as usize;
            let top = chars[2];
            mapping[bottom1][bottom2].push(top);
        }
        
        // 递归函数，用于构建金字塔

        fn build_pyramid(row: &Vec<char>, next_row: &mut Vec<char>, index: usize, mapping: &Vec<Vec<Vec<char>>>) -> bool {
            if row.len() == 1 {
                return true;
            }
            if index == row.len() - 1 {
                return build_pyramid(next_row, &mut vec![], 0, mapping);
            }
            let bottom1 = row[index] as usize - 'A' as usize;
            let bottom2 = row[index + 1] as usize - 'A' as usize;
            for &top in &mapping[bottom1][bottom2] {
                next_row.push(top);
                if build_pyramid(row, next_row, index + 1, mapping) {
                    return true;
                }
                next_row.pop();
            }
            false

        }
        
        let bottom_chars: Vec<char> = bottom.chars().collect();
        build_pyramid(&bottom_chars, &mut vec![], 0, &mapping)
    }
}
