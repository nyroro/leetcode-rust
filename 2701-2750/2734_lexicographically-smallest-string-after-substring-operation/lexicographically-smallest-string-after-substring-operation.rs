


impl Solution {
    pub fn smallest_string(s: String) -> String {
        let mut result = s.clone().into_bytes();
        let n = result.len();
        for i in 0..n {
            if result[i] != b'a' {
                result[i] -= 1;
                for j in i + 1..n {
                    result[j] = b'z';
                }
                return String::from_utf8(result).unwrap();
            }
        }
        result[n - 1] = b'z';
        String::from_utf8(result).unwrap()
    }
}
