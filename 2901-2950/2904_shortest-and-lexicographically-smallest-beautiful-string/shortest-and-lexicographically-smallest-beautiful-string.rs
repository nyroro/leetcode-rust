
impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        let mut i = 0;
        let mut j = 0;
        let mut x = 0;

        let mut ret = String::new();
        let s = s.chars().collect::<Vec<char>>();
        while i < s.len() && s[i] == '0' {
            i += 1;
        }
        x = 1;
        if i == s.len() {
            return String::new();
        }
        if x == k {
            ret.push(s[i]);
            return ret;
        }
        for j in (i + 1)..s.len() {
            if s[j] == '1' {
                x += 1;
            }
            if x == k {
                if ret.is_empty() {
                    ret = s[i..=j].iter().collect();
                } else if ret.len() > (j - i + 1) {
                    ret = s[i..=j].iter().collect();
                } else if ret.len() == (j - i + 1) && ret > s[i..=j].iter().collect() {
                    ret = s[i..=j].iter().collect();
                }
                i += 1;
                x -= 1;
                while i < j && s[i] != '1' {
                    i += 1;
                }
            }
        }
        ret

    }
}
