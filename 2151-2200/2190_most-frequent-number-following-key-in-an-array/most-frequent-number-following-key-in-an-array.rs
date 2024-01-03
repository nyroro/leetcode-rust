
impl Solution {
    pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
        use std::collections::HashMap;
        
        let mut count_map: HashMap<i32, i32> = HashMap::new();
        let mut max_count = 0;
        let mut result = 0;
        
        for i in 0..nums.len() - 1 {
            if nums[i] == key {
                let target = nums[i + 1];
                let count = count_map.entry(target).or_insert(0);
                *count += 1;
                if *count > max_count {
                    max_count = *count;
                    result = target;
                }
            }
        }
        
        result

    }
}
