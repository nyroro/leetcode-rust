
impl Solution {
    pub fn minimum_moves(s: String) -> i32 {
        let mut moves = 0;
        let mut chars: Vec<char> = s.chars().collect();
        let n = chars.len();

        let mut i = 0;
        while i < n {
            if i + 2 < n && chars[i] == 'X' && chars[i + 1] == 'X' && chars[i + 2] == 'X' {
                moves += 1;
                i += 3;
            } else {
                i += 1;
            }
        }

        moves

    }
}
