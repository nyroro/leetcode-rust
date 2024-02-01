
impl Solution {
    pub fn min_swaps_couples(mut row: Vec<i32>) -> i32 {
        let mut pos_map = std::collections::HashMap::new();
        let mut swaps = 0;
        
        for (i, &val) in row.iter().enumerate() {
            pos_map.insert(val, i);
        }
        
        for i in (0..row.len()).step_by(2) {
            let x = row[i];
            let y = if row[i] % 2 == 0 { row[i] + 1 } else { row[i] - 1 };
            let partner_pos = pos_map[&y];
            
            if partner_pos != i + 1 {
                row.swap(i + 1, partner_pos);
                pos_map.insert(row[i + 1], i + 1);
                pos_map.insert(row[partner_pos], partner_pos);
                swaps += 1;
            }
        }
        
        swaps

    }
}
