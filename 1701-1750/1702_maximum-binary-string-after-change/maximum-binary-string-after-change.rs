
impl Solution {
    pub fn maximum_binary_string(binary: String) -> String {
        let mut chars: Vec<char> = binary.chars().collect();
        let n = chars.len();
        let mut zeros = 0;
        let mut ones = 0;
        let mut first_zero = n;
        
        for i in 0..n {
            if chars[i] == '0' {
                zeros += 1;
                first_zero = std::cmp::min(first_zero, i);
            } else if chars[i] == '1' {
                ones += 1;
            }
        }
        
        if zeros == 0 {
            return binary;
        }
        
        for i in first_zero..n {
            if chars[i] == '0' {
                chars[i] = '1';
            }
        }
        
        chars[first_zero + zeros - 1] = '0';
        
        chars.iter().collect()
    }
}
