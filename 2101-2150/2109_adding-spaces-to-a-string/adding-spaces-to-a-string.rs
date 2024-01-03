
impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut result = String::new();
        let mut spaces_iter = spaces.iter().copied();
        let mut space_index = spaces_iter.next();

        for (i, c) in s.chars().enumerate() {
            if space_index.is_some() && i == space_index.unwrap() as usize {
                result.push(' ');
                space_index = spaces_iter.next();
            }
            result.push(c);
        }

        result

    }
}
