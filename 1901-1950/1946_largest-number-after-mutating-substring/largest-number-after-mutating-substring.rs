
impl Solution {
    pub fn maximum_number(num: String, change: Vec<i32>) -> String {
        let mut chars: Vec<char> = num.chars().collect();
        let mut changed = false;

        for i in 0..chars.len() {
            let digit = chars[i].to_digit(10).unwrap() as usize;
            if change[digit] > digit as i32 && (!changed || change[digit] >= digit as i32) {
                chars[i] = std::char::from_digit(change[digit] as u32, 10).unwrap();
                changed = true;
            } else if changed {
                break;
            }
        }

        chars.into_iter().collect()
    }
}
