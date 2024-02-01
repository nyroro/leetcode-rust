
use std::collections::HashMap;



impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut s_map: HashMap<char, i32> = HashMap::new();
        let mut t_map: HashMap<char, i32> = HashMap::new();

        // Count occurrences of each character in s and t

        for ch in s.chars() {
            *s_map.entry(ch).or_insert(0) += 1;
        }

        for ch in t.chars() {
            *t_map.entry(ch).or_insert(0) += 1;
        }

        // Calculate the total number of steps required

        let mut steps = 0;
        for ch in b'a'..=b'z' {
            let char_count_s = s_map.get(&(ch as char)).unwrap_or(&0);
            let char_count_t = t_map.get(&(ch as char)).unwrap_or(&0);
            steps += (char_count_s - char_count_t).abs();
        }

        steps

    }
}

fn main() {
    let s = "leetcode".to_string();
    let t = "coats".to_string();
    println!("{}", Solution::min_steps(s, t));  // Output: 7

}
