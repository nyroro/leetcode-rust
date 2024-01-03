
impl Solution {
    pub fn original_digits(s: String) -> String {
        let mut counts = vec![0; 10];
        let mut char_counts = vec![0; 26];
        for c in s.chars() {
            char_counts[(c as u8 - b'a') as usize] += 1;
        }
        counts[0] = char_counts[('z' as u8 - b'a') as usize];
        counts[2] = char_counts[('w' as u8 - b'a') as usize];
        counts[4] = char_counts[('u' as u8 - b'a') as usize];
        counts[6] = char_counts[('x' as u8 - b'a') as usize];
        counts[8] = char_counts[('g' as u8 - b'a') as usize];
        counts[3] = char_counts[('h' as u8 - b'a') as usize] - counts[8];
        counts[5] = char_counts[('f' as u8 - b'a') as usize] - counts[4];
        counts[7] = char_counts[('s' as u8 - b'a') as usize] - counts[6];
        counts[9] = char_counts[('i' as u8 - b'a') as usize] - counts[5] - counts[6] - counts[8];
        counts[1] = char_counts[('n' as u8 - b'a') as usize] - 2 * counts[9] - counts[7];
        let mut result = String::new();
        for i in 0..10 {
            for _ in 0..counts[i] {
                result.push((i as u8 + b'0') as char);
            }
        }
        result

    }
}
