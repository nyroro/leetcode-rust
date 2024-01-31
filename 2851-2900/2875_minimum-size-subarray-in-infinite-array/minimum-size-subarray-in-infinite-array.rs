
impl Solution {
    pub fn min_size_subarray(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let s: i32 = nums.iter().sum();
        
        if target % s == 0 {
            return ((target / s) * n as i32) as i32;
        }
        
        let mut x = ((target / s) * n as i32) as i32;
        let mut k = target - (s * (target / s));
        
        let mut extended_array = nums.clone();
        extended_array.extend(&nums);
        let n = extended_array.len();
        
        let mut i = 0;
        let mut j = 0;
        let mut s = 0;
        let mut ans = std::i32::MAX;
        
        while i < n && j < n {
            s += extended_array[j];
            while i < n && s > k {
                s -= extended_array[i];
                i += 1;
            }
            while i < n && s == k {
                ans = ans.min((j - i + 1) as i32);
                s -= extended_array[i];
                i += 1;
            }
            j += 1;
        }
        
        if ans == std::i32::MAX {
            return -1;
        }
        
        return ans + x;
    }
}
