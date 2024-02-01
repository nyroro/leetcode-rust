
impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut digits = n.to_string().chars().collect::<Vec<char>>();
        let len = digits.len();
        
        let mut i = (len - 2) as i32;
        while i >= 0 && digits[i as usize] >= digits[(i + 1) as usize] {
            i -= 1;
        }
        
        if i < 0 {
            return -1;
        }
        
        let mut j = len - 1;
        while j > i as usize && digits[j] <= digits[i as usize] {
            j -= 1;
        }
        
        digits.swap(i as usize, j);
        digits[(i as usize + 1)..].sort();
        
        let result = digits.iter().collect::<String>().parse::<i64>();
        match result {
            Ok(num) => {
                if num > i32::MAX as i64 {
                    return -1;
                }
                num as i32

            }
            Err(_) => -1,
        }
    }
}
