
impl Solution {
    pub fn minimum_partition(s: String, k: i32) -> i32 {
        let mut tmp = String::new();
        let mut cnt = 0;

        for c in s.chars() {
            tmp.push(c);

            match tmp.parse::<i64>() {
                Ok(val) => {
                    if val > k as i64 {
                        tmp = c.to_string();
                        if c.to_string().parse::<i64>().unwrap() > k as i64 {
                            return -1;
                        }
                        cnt += 1;
                    }
                }
                Err(_) => {
                    // Handle parsing error

                    return -1;
                }
            }
        }

        cnt + 1

    }
}
