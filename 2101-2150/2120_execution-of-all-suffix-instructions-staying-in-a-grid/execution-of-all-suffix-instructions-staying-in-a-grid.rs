
impl Solution {
    pub fn execute_instructions(n: i32, start_pos: Vec<i32>, s: String) -> Vec<i32> {
        let mut result = Vec::new();
        let directions: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (-1, 0), (1, 0)]; // 右、左、上、下

        for i in 0..s.len() {
            let mut row = start_pos[0];
            let mut col = start_pos[1];
            let mut count = 0;
            for c in s[i..].chars() {
                let (dr, dc) = directions[match c {
                    'R' => 0,
                    'L' => 1,
                    'U' => 2,
                    'D' => 3,
                    _ => unreachable!(),
                }];
                if row + dr >= 0 && row + dr < n && col + dc >= 0 && col + dc < n {
                    row += dr;
                    col += dc;
                    count += 1;
                } else {
                    break;
                }
            }
            result.push(count);
        }
        result

    }
}
