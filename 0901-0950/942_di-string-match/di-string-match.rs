
impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut low = 0;
        let mut high = s.len() as i32;
        let mut perm = Vec::new();

        for ch in s.chars() {
            if ch == 'I' {
                perm.push(low);
                low += 1;
            } else if ch == 'D' {
                perm.push(high);
                high -= 1;
            }
        }

        perm.push(low);

        perm

    }
}
