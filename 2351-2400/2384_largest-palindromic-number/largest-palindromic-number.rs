
use std::collections::HashMap;

impl Solution {
    pub fn largest_palindromic(num: String) -> String {
        let mut counter = HashMap::new();
        for c in num.chars() {
            *counter.entry(c).or_insert(0) += 1;
        }

        let mut s = Vec::new();
        let mut t = String::new();

        for i in (0..=9).rev() {
            let v = counter.get(&char::from_digit(i, 10).unwrap_or('0')).unwrap_or(&0) / 2;
            let v2 = counter.get(&char::from_digit(i, 10).unwrap_or('0')).unwrap_or(&0) % 2;
            for _ in 0..v {
                s.push(char::from_digit(i, 10).unwrap());
            }
            if v2 > 0 && t.is_empty() {
                t = i.to_string();
            }
        }

        if s.is_empty() || s[0] == '0' {
            if !t.is_empty() {
                return t;
            } else {
                return "0".to_string();
            }
        } else {
            let mut result = s.iter().collect::<String>();
            result.push_str(&t);
            result.push_str(&s.iter().rev().collect::<String>());
            return result;
        }
    }
}
