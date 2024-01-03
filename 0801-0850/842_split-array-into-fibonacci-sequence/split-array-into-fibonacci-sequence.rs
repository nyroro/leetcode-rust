
impl Solution {
    pub fn split_into_fibonacci(num: String) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        Self::backtrack(&num, &mut result, 0);
        result

    }
    
    fn backtrack(num: &str, result: &mut Vec<i32>, index: usize) -> bool {
        if index == num.len() && result.len() >= 3 {
            return true;
        }
        
        for i in index..num.len() {
            if num.chars().nth(index) == Some('0') && i > index {
                break;
            }
            
            let current: i64 = num[index..=i].parse().unwrap();
            if current > i32::MAX as i64 {
                break;
            }
            
            let size = result.len();
            if size >= 2 && current > (result[size - 1] as i64) + (result[size - 2] as i64) {
                break;
            }
            
            if size <= 1 || current == (result[size - 1] as i64) + (result[size - 2] as i64) {
                result.push(current as i32);
                if Self::backtrack(num, result, i + 1) {
                    return true;
                }
                result.pop();
            }
        }
        
        false

    }
}
