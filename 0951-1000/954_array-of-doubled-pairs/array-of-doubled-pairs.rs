
impl Solution {
    pub fn can_reorder_doubled(arr: Vec<i32>) -> bool {
        use std::collections::HashMap;
        
        let mut count = HashMap::new();
        let mut arr = arr;
        
        arr.sort_unstable_by_key(|&x| x.abs());
        
        for &num in &arr {
            *count.entry(num).or_insert(0) += 1;
        }
        
        for &num in &arr {
            if count[&num] == 0 {
                continue;
            }
            if let Some(&mut v) = count.get_mut(&(2 * num)) {
                if v == 0 {
                    return false;
                }
                *count.get_mut(&num).unwrap() -= 1;
                *count.get_mut(&(2 * num)).unwrap() -= 1;
            } else {
                return false;
            }
        }
        
        true

    }
}
