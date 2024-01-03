
impl Solution {
    pub fn clumsy(n: i32) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        stack.push(n);
        let mut i = n - 1;
        let mut index = 0;
        
        while i > 0 {
            if index % 4 == 0 {
                let top = stack.pop().unwrap();
                stack.push(top * i);
            } else if index % 4 == 1 {
                let top = stack.pop().unwrap();
                stack.push(top / i);
            } else if index % 4 == 2 {
                stack.push(i);
            } else {
                stack.push(-i);
            }
            
            i -= 1;
            index += 1;
        }
        
        stack.iter().sum()
    }
}
