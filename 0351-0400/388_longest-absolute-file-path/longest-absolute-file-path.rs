
impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let lines: Vec<&str> = input.split('\n').collect();
        let mut stack: Vec<usize> = vec![0];
        let mut max_len = 0;
        
        for line in lines {
            let level = line.chars().take_while(|&c| c == '\t').count();
            let name = line.chars().skip(level).collect::<String>();
            
            while level + 1 < stack.len() {
                stack.pop();
            }
            
            let len = stack.last().unwrap() + name.len();
            stack.push(len);
            
            if name.contains('.') {
                max_len = max_len.max(len + level);
            }
        }
        
        max_len as i32

    }
}
