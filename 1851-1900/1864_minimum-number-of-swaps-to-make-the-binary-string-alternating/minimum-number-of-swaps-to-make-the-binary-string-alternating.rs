


impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let s_chars: Vec<char> = s.chars().collect();
        let mut zeros = 0;
        let mut ones = 0;

        for &c in s_chars.iter() {
            match c {
                '0' => zeros += 1,
                '1' => ones += 1,
                _ => (),
            }
        }

        if (zeros as i32 - ones as i32).abs() > 1 {
            return -1;
        }

        let length = s_chars.len();
        let mut swap0 = 0;
        let mut swap1 = 0;

        if length % 2 == 0 {
            for i in 0..length {
                let c = s_chars[i];
                if i % 2 == 0 {
                    if c == '1' {
                        swap0 += 1;
                    } else {
                        swap1 += 1;
                    }
                } else {
                    if c == '0' {
                        swap0 += 1;
                    } else {
                        swap1 += 1;
                    }
                }
            }
            return std::cmp::min(swap0, swap1) / 2;
        } else {
            let mut swap = 0;
            if zeros > ones {
                for i in 0..length {
                    let c = s_chars[i];
                    if i % 2 == 0 {
                        if c == '1' {
                            swap += 1;
                        }
                    } else {
                        if c == '0' {
                            swap += 1;
                        }
                    }
                }
            } else {
                for i in 0..length {
                    let c = s_chars[i];
                    if i % 2 == 0 {
                        if c == '0' {
                            swap += 1;
                        }
                    } else {
                        if c == '1' {
                            swap += 1;
                        }
                    }
                }
            }
            return swap / 2;
        }
    }
}
