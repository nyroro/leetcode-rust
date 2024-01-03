
impl Solution {
    pub fn find_the_string(lcp: Vec<Vec<i32>>) -> String {
        let n = lcp.len();
        let mut word = vec![' '; n];

        for c in b'a'..=b'z' {
            let mut i = 0;
            while i < n && word[i] != ' ' {
                i += 1;
            }
            if i == n {
                break;
            }
            for j in i..n {
                if lcp[i][j] > 0 {
                    word[j] = c as char;
                }
            }
        }

        for i in 0..n {
            if word[i] == ' ' {
                return String::new();
            }
        }

        for i in (0..n).rev() {
            for j in (0..n).rev() {
                if word[i] == word[j] {
                    if i == n - 1 || j == n - 1 {
                        if lcp[i][j] != 1 {
                            return String::new();
                        }
                    } else if lcp[i][j] != lcp[i + 1][j + 1] + 1 {
                        return String::new();
                    }
                } else if lcp[i][j] > 0 {
                    return String::new();
                }
            }
        }

        word.iter().collect()
    }
}
