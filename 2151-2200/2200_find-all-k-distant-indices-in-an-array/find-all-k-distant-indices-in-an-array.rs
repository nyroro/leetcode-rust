
impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let mut result = Vec::new();
        
        for i in 0..nums.len() {
            let mut found = false;
            for j in (i as i32 - k)..=(i as i32 + k) {
                if j >= 0 && j < nums.len() as i32 && nums[j as usize] == key {
                    found = true;
                    break;
                }
            }
            if found {
                result.push(i as i32);
            }
        }
        
        result

    }
}
