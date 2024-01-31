
impl Solution {
    pub fn minimum_beautiful_substrings(s: String) -> i32 {
        let bins: Vec<String> = (0..7).map(|i| format!("{:b}", 5i32.pow(i))).collect();

        fn is_beautiful(s: &str, bins: &Vec<String>) -> bool {
            if s.replace("_", "").is_empty() {
                return true;
            }

            for i in bins {
                if s.contains(i) {
                    if is_beautiful(&s.replace(i, "_"), bins) {
                        return true;
                    }
                }
            }
            false

        }

        fn calculate(s: &str, bins: &Vec<String>, ans: &mut i32) {
            for i in bins {
                if s.contains(i) {
                    let updated_s = s.replace(i, "_");
                    calculate(&updated_s, bins, ans);
                }
            }
            if s.replace("_", "").is_empty() {
                *ans = (*ans).min(s.len() as i32);
            }
        }

        let mut ans = i32::MAX;
        calculate(&s, &bins, &mut ans);
        if ans == i32::MAX {
            -1

        } else {
            ans

        }
    }
}
