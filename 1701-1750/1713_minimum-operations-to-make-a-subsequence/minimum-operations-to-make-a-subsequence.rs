
use std::collections::HashMap;

impl Solution {
    pub fn min_operations(target: Vec<i32>, arr: Vec<i32>) -> i32 {
        let mut index_map: HashMap<i32, usize> = HashMap::new();
        for (i, &num) in target.iter().enumerate() {
            index_map.insert(num, i);
        }
        
        let mut indices: Vec<usize> = Vec::new();
        for &num in &arr {
            if let Some(&index) = index_map.get(&num) {
                indices.push(index);
            }
        }
        
        let mut lis: Vec<usize> = Vec::new();
        for &index in &indices {
            if lis.is_empty() || index > *lis.last().unwrap() {
                lis.push(index);
            } else {
                let pos = lis.binary_search(&index).unwrap_or_else(|x| x);
                lis[pos] = index;
            }
        }
        
        (target.len() - lis.len()) as i32

    }
}
