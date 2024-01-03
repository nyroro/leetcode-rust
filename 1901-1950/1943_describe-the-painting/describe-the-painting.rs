
impl Solution {
    pub fn split_painting(segments: Vec<Vec<i32>>) -> Vec<Vec<i64>> {
        let mut color_map: std::collections::HashMap<i32, i64> = std::collections::HashMap::new();
        let mut painting: Vec<Vec<i64>> = Vec::new();
        
        // 遍历所有的颜色段，更新哈希表中的颜色和总和

        for segment in segments {
            let start = segment[0];
            let end = segment[1];
            let color = segment[2];
            
            // 更新哈希表中的颜色和总和

            *color_map.entry(start).or_insert(0) += color as i64;
            *color_map.entry(end).or_insert(0) -= color as i64;
        }
        
        let mut positions: Vec<i32> = color_map.keys().cloned().collect();
        positions.sort();
        
        let mut prev_position = positions[0];
        let mut prev_sum = 0;
        
        // 根据哈希表的结果生成最终的颜色段

        for position in positions {
            let sum = prev_sum + color_map[&position];
            
            if prev_sum > 0 {
                painting.push(vec![prev_position as i64, position as i64, prev_sum]);
            }
            
            prev_position = position;
            prev_sum = sum;
        }
        
        painting

    }
}
