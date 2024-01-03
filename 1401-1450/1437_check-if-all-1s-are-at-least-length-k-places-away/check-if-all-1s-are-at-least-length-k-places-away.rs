
impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut prev: Option<usize> = None;
        
        for (i, &num) in nums.iter().enumerate() {
            if num == 1 {
                if let Some(prev_index) = prev {
                    if (i - prev_index - 1) < k as usize {
                        return false;
                    }
                }
                prev = Some(i);
            }
        }
        
        true

    }
}
