
impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let mut result = String::new();
        let mut i = 0;
        let bytes = s.as_bytes();
        
        while i < bytes.len() {
            if i + 2 < bytes.len() && bytes[i + 2] == b'#' {
                let num = (bytes[i] - b'0') * 10 + (bytes[i + 1] - b'0');
                result.push((b'a' + num - 1) as char);
                i += 3;
            } else {
                let num = bytes[i] - b'0';
                result.push((b'a' + num - 1) as char);
                i += 1;
            }
        }
        
        result

    }
}
