
use std::collections::HashMap; // 导入HashMap模块


impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut sorted_arr = arr.clone();
        sorted_arr.sort();
        
        let mut rank_map = HashMap::new();
        let mut rank = 1;
        
        for num in sorted_arr {
            rank_map.entry(num).or_insert_with(|| {
                let current_rank = rank;
                rank += 1;
                current_rank

            });
        }
        
        let mut result = Vec::new();
        
        for num in arr {
            result.push(*rank_map.get(&num).unwrap());
        }
        
        result

    }
}
