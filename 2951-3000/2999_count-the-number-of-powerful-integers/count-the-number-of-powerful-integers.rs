
impl Solution {
    pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
        let mut cnt: Vec<i64> = vec![0; 16];

        fn count(n: &str, s: &str, cnt: &Vec<i64>, limit: i32) -> i64 {
            let mut res = cnt[n.len() - 1];
            let mut i = 0;
            let sz = n.len().saturating_sub(s.len());
            while i <= sz {
                res += if i == sz {
                    if n[i..].parse::<i64>().unwrap() >= s.parse::<i64>().unwrap() { 1 } else { 0 }
                } else {
                    cnt[n.len() - i - 1] * (std::cmp::min(limit as i64, n.chars().nth(i).unwrap().to_digit(10).unwrap() as i64 - 1) + (i > 0) as i64)
                };
                i += 1;
                if i > sz || n.chars().nth(i - 1).unwrap().to_digit(10).unwrap() as i64 > limit as i64 {
                    break;
                }
            }
            res

        }

        for i in (s.len()..16) {
            cnt[i] = if i == s.len() {
                1

            } else {
                cnt[i - 1] * (limit as i64 + 1)
            };
        }

        count(&finish.to_string(), &s, &cnt, limit) - count(&(start - 1).to_string(), &s, &cnt, limit)
    }
}
