


impl Solution {
    pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
        let n = (encoded_text.len() as i32) / rows;
        let mut matrix: Vec<Vec<char>> = vec![vec![' '; n as usize]; rows as usize];

        let mut iter = encoded_text.chars().enumerate().cycle();
        for j in 0..rows {
            for i in 0..n {
                if let Some((_, ch)) = iter.next() {
                    matrix[j as usize][i as usize] = ch;
                }
            }
        }

        let mut original_text = String::new();
        for i in 0..n {
            for j in 0..rows {
                if i + j < n {
                    original_text.push(matrix[j as usize][(i + j) as usize]);
                } else {
                    break;
                }
            }
        }

        original_text.trim_end().to_string()
    }
}
