
impl Solution {
    pub fn largest_time_from_digits(arr: Vec<i32>) -> String {
        let mut max_time = -1;
        let mut result = String::new();
        let mut visited = vec![false; 4];
        let mut current_time = [0; 4];
        
        Self::backtrack(&arr, &mut visited, &mut current_time, &mut max_time, 0);
        
        if max_time == -1 {
            return result;
        }
        
        result.push_str(&format!("{:02}:{:02}", max_time / 60, max_time % 60));
        result

    }
    
    fn backtrack(arr: &Vec<i32>, visited: &mut Vec<bool>, current_time: &mut [i32; 4], max_time: &mut i32, pos: usize) {
        if pos == 4 {
            let hours = current_time[0] * 10 + current_time[1];
            let minutes = current_time[2] * 10 + current_time[3];
            
            if hours < 24 && minutes < 60 {
                let time = hours * 60 + minutes;
                *max_time = (*max_time).max(time);
            }
            
            return;
        }
        
        for i in 0..4 {
            if !visited[i] {
                visited[i] = true;
                current_time[pos] = arr[i];
                Self::backtrack(arr, visited, current_time, max_time, pos + 1);
                visited[i] = false;
            }
        }
    }
}
