
use std::collections::BTreeSet;

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut set: BTreeSet<i32> = BTreeSet::new();
        
        for num in nums {
            set.insert(num);
            
            if set.len() > 3 {
                set.remove(&set.iter().next().unwrap().clone());
            }
        }
        
        if set.len() < 3 {
            *set.iter().next_back().unwrap()
        } else {
            *set.iter().next().unwrap()
        }
    }
}
