
impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let n = nums.len();
        let mut result = String::new();
        
        for i in 0..(1 << n) {
            let binary = format!("{:0width$b}", i, width = n);
            if !nums.contains(&binary) {
                result = binary;
                break;
            }
        }
        
        result

    }
}
