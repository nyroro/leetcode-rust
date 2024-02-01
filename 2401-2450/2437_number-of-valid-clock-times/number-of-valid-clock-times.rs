
impl Solution {
    pub fn count_time(time: String) -> i32 {
        let mut count = 0;
        let time_chars: Vec<char> = time.chars().collect();
        
        for h1 in 0..10 {
            if time_chars[0] != '?' && (time_chars[0] as u8 - b'0') != h1 {
                continue;
            }
            for h2 in 0..10 {
                if time_chars[1] != '?' && (time_chars[1] as u8 - b'0') != h2 {
                    continue;
                }
                if h1 * 10 + h2 >= 24 {
                    continue;
                }
                for m1 in 0..10 {
                    if time_chars[3] != '?' && (time_chars[3] as u8 - b'0') != m1 {
                        continue;
                    }
                    for m2 in 0..10 {
                        if time_chars[4] != '?' && (time_chars[4] as u8 - b'0') != m2 {
                            continue;
                        }
                        if m1 * 10 + m2 < 60 {
                            count += 1;
                        }
                    }
                }
            }
        }
        
        count

    }
}
