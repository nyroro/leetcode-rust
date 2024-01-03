
impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        let mut num = num.chars().collect::<Vec<char>>();
        let mut i = num.len() as i32 - 1;
        while i >= 0 {
            if (num[i as usize] as i32 - '0' as i32) % 2 != 0 {
                return num[..=i as usize].iter().collect();
            }
            i -= 1;
        }
        "".to_string()
    }
}
