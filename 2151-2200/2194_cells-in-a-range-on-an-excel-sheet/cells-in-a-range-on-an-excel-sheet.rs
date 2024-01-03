
impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        let mut result = Vec::new();
        let cols: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
        
        let start_col = s.chars().nth(0).unwrap();
        let start_row = s.chars().nth(1).unwrap().to_digit(10).unwrap() as usize;
        let end_col = s.chars().nth(3).unwrap();
        let end_row = s.chars().nth(4).unwrap().to_digit(10).unwrap() as usize;
        
        for col in cols.iter() {
            if *col >= start_col && *col <= end_col {
                for row in start_row..=end_row {
                    result.push(format!("{}{}", col, row));
                }
            }
        }
        
        result.sort();
        result

    }
}
