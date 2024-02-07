
impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let mut cum_shifts = vec![0i64; s.len() + 1];

        for shift in &shifts {
            let start = shift[0] as usize;
            let end = shift[1] as usize;
            let direction = shift[2];

            if direction == 0 {
                cum_shifts[start] -= 1;
                cum_shifts[end + 1] += 1;
            } else {
                cum_shifts[start] += 1;
                cum_shifts[end + 1] -= 1;
            }
        }

        let mut cum_sum: i64 = 0;
        let mut modified_s = String::new();

        for (i, c) in s.chars().enumerate() {
            cum_sum += cum_shifts[i] as i64;
            let new_code = ((((c as u8 as i64 - b'a' as i64 + cum_sum) % 26 + 26) % 26 + b'a' as i64) as u8) as char;
            modified_s.push(new_code);
        }

        modified_s

    }
}
