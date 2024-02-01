


impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let mut max_length = 0;
        Solution::backtrack(0, &arr, "".to_string(), &mut max_length);
        max_length

    }

    fn is_unique(s: &str) -> bool {
        let mut seen = [false; 26];
        for ch in s.chars() {
            let idx = (ch as u8 - b'a') as usize;
            if seen[idx] {
                return false;
            }
            seen[idx] = true;
        }
        true

    }

    fn backtrack(start: usize, arr: &Vec<String>, current: String, max_length: &mut i32) {
        if Solution::is_unique(&current) {
            *max_length = (*max_length).max(current.len() as i32);
        } else {
            return;
        }

        for i in start..arr.len() {
            let new_str = format!("{}{}", current, arr[i]);
            Solution::backtrack(i + 1, arr, new_str, max_length);
        }
    }
}
