
impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let n = nums.len();
        let mut result = String::new();
        
        for i in 0..n {
            if nums.contains(&format!("{:0width$b}", i, width = n)) {
                continue;
            } else {
                result = format!("{:0width$b}", i, width = n);
                break;
            }
        }
        
        result

    }
}
