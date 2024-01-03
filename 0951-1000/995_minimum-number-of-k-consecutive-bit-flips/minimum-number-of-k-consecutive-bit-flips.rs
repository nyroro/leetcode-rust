
use std::collections::VecDeque;

impl Solution {
    pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
        let mut flips = 0;
        let mut queue = VecDeque::new();
        
        for i in 0..nums.len() {
            if !queue.is_empty() && queue[0] == i as i32 - k {
                queue.pop_front();
            }
            
            if queue.len() % 2 == nums[i] as usize {
                if i + k as usize > nums.len() {
                    return -1;
                }
                
                flips += 1;
                queue.push_back(i as i32);
            }
        }
        
        flips

    }
}
