
impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        if k == 1 {
            let mut result = s.clone();
            let s_chars: Vec<char> = s.chars().collect();
            for i in 1..s.len() {
                let rotated = s_chars.iter().cycle().skip(i).take(s.len()).collect::<String>();
                if rotated < result {
                    result = rotated;
                }
            }
            result

        } else {
            let mut s_chars: Vec<char> = s.chars().collect();
            s_chars.sort();
            s_chars.iter().collect::<String>()
        }
    }
}
