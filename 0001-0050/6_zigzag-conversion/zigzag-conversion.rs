
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let n = s.len();
        if num_rows == 1 || n <= num_rows as usize {
            return s;
        }
        
        let mut rows: Vec<String> = vec![String::new(); num_rows as usize];
        let mut cur_row = 0;
        let mut going_down = false;
        
        for c in s.chars() {
            rows[cur_row].push(c);
            if cur_row == 0 || cur_row == num_rows as usize - 1 {
                going_down = !going_down;
            }
            cur_row = if going_down { cur_row + 1 } else { cur_row - 1 };
        }
        
        rows.join("")
    }
}
