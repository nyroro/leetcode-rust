
use std::collections::HashMap;

impl Solution {
    pub fn max_rep_opt1(text: String) -> i32 {
        let mut arr: Vec<(char, usize)> = Vec::new();
        let mut now: char = ' ';
        let mut cnt: usize = 0;
        
        for c in text.chars() {
            if c != now {
                if now != ' ' {
                    arr.push((now, cnt));
                }
                now = c;
                cnt = 1;
            } else {
                cnt += 1;
            }
        }
        if cnt > 0 {
            arr.push((now, cnt));
        }

        let mut char_count = HashMap::new();
        for c in text.chars() {
            *char_count.entry(c).or_insert(0) += 1;
        }

        let mut ret: usize = 0;
        for (a, b) in &arr {
            if *char_count.get(&a).unwrap_or(&0) > *b {
                ret = ret.max(*b + 1);
            } else {
                ret = ret.max(*b);
            }
        }

        for i in 0..arr.len() {
            if i + 2 < arr.len() && arr[i].0 == arr[i + 2].0 && arr[i + 1].1 == 1 {
                let val = arr[i].1 + arr[i + 2].1 + 1;
                let val = val.min(*char_count.get(&arr[i].0).unwrap_or(&0));
                ret = ret.max(val);
            }
        }

        ret as i32

    }
}
