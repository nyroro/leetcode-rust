


impl Solution {
    pub fn count_texts(pressed_keys: String) -> i64 {
        let s = pressed_keys.chars().collect::<Vec<char>>();

        fn dp(i: usize, s: &Vec<char>, memo: &mut Vec<Option<i64>>) -> i64 {
            if i == s.len() {
                return 1;
            }

            if let Some(val) = memo[i] {
                return val;
            }

            let mut ans = 0;
            let mut j = i;
            let mut prev = ' ';
            while j < std::cmp::min(i + 4, s.len()) {
                if prev == ' ' || prev == s[j] {
                    if s[j] == '2' || s[j] == '3' || s[j] == '4' || s[j] == '5' || s[j] == '6' || s[j] == '8' {
                        if j - i < 3 {
                            ans += dp(j + 1, s, memo);
                        }
                    } else {
                        ans += dp(j + 1, s, memo);
                    }
                } else {
                    break;
                }
                prev = s[j];
                j += 1;
            }
            let result = ans % (10i64.pow(9) + 7);
            memo[i] = Some(result);
            result

        }

        let mut memo = vec![None; s.len()];
        dp(0, &s, &mut memo)
    }
}
