
impl Solution {
    pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = [0; 32];
        
        for num in nums {
            for i in 0..32 {
                if (num & (1 << i)) != 0 {
                    count[i] += 1;
                }
            }
        }
        
        let mut result = 0;
        for i in 0..32 {
            if count[i] >= k {
                result |= 1 << i;
            }
        }
        
        result

    }
}
