
use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut result = Vec::new();
        let mut deque = VecDeque::new();
        
        for i in 0..nums.len() {
            // 检查队列的头部元素是否超出窗口范围

            if !deque.is_empty() && deque.front().unwrap() == &(i - k) {
                deque.pop_front();
            }
            
            // 检查队列的尾部元素是否小于当前元素

            while !deque.is_empty() && nums[*deque.back().unwrap()] < nums[i] {
                deque.pop_back();
            }
            
            // 将当前元素添加到队列的尾部

            deque.push_back(i);
            
            // 如果窗口已经形成，则将队列的头部元素添加到结果数组中

            if i >= k - 1 {
                result.push(nums[*deque.front().unwrap()]);
            }
        }
        
        result

    }
}
