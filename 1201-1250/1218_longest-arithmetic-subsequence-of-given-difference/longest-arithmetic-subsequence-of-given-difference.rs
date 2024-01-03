
impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        use std::collections::HashMap;
        
        let mut map = HashMap::new();
        let mut max_length = 0;
        
        for &num in arr.iter() {
            let prev = num - difference;
            let length = *map.get(&prev).unwrap_or(&0) + 1;
            map.insert(num, length);
            max_length = max_length.max(length);
        }
        
        max_length

    }
}
