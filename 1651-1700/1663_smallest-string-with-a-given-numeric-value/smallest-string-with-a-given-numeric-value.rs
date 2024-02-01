
impl Solution {
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        let mut result = vec!['a'; n as usize];
        let mut total = n;

        for i in (0..n as usize).rev() {
            let diff = k - total + 1;
            if diff > 0 {
                let diff = diff.min(25);
                result[i] = (b'a' + diff as u8) as char;
                total += diff - 1;
            }
            if total == k {
                break;
            }
        }

        result.into_iter().collect()
    }
}
