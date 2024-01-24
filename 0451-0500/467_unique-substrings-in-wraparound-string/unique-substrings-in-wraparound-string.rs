
impl Solution {
    pub fn find_substring_in_wrapround_string(s: String) -> i32 {
        let mut ret = 0;
        let mut prev_ret = 0;
        let mut arr = vec![0; 26];
        let s_bytes = s.as_bytes();

        for i in 0..s.len() {
            if i > 0 && (s_bytes[i - 1] as i32 + 1 - 'a' as i32) % 26 == (s_bytes[i] as i32 - 'a' as i32) {
                prev_ret += 1;
            } else {
                prev_ret = 1;
            }
            let index = (s_bytes[i] - b'a') as usize;
            if arr[index] < prev_ret {
                ret += prev_ret - arr[index];
                arr[index] = prev_ret;
            }
        }
        ret

    }
}
