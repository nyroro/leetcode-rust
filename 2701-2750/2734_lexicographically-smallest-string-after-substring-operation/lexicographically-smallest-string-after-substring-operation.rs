


impl Solution {
    pub fn smallest_string(s: String) -> String {
        let mut result = s.clone().into_bytes();
        let n = result.len();
        let mut i = 0;
        while i < n && result[i] == b'a' {
            i += 1;
        }
        if i == n {
            result[n - 1] = b'z';
        } else {
            let mut j = i;
            while j < n && result[j] != b'a' {
                result[j] -= 1;
                j += 1;
            }
        }
        String::from_utf8(result).unwrap()
    }
}
