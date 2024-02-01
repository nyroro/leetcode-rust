
impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut shift_sum = 0;

        for i in (0..n).rev() {
            shift_sum = (shift_sum + shifts[i]) % 26;
            let shift = (chars[i] as u8 - b'a') as i32;
            chars[i] = ((shift + shift_sum) % 26 + b'a') as char;
        }

        chars.into_iter().collect()
    }
}
