
impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut result = Vec::new();
        
        if n % 2 != 0 {
            result.push(0);
        }
        
        let mut num = 1;
        while result.len() < n as usize {
            result.push(num);
            result.push(-num);
            num += 1;
        }
        
        result

    }
}
