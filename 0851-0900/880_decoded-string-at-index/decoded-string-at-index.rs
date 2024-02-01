


impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        let mut stack: Vec<(char, i64)> = Vec::new();
        let mut size: i64 = 0;

        for c in s.chars() {
            if c.is_digit(10) {
                let digit = c.to_digit(10).unwrap() as i64;
                size *= digit;
            } else {
                size += 1;
            }
            stack.push((c, size));
        }

        let mut k = k as i64;
        while let Some((c, count)) = stack.pop() {
            k %= count;
            if k == 0 && c.is_alphabetic() {
                return c.to_string();
            }
        }

        String::new()
    }
}

fn main() {
    let s1 = "leet2code3".to_string();
    let k1 = 10;
    let result1 = Solution::decode_at_index(s1, k1);
    println!("Result 1: {}", result1);  // Output: "o"

    let s2 = "ha22".to_string();
    let k2 = 5;
    let result2 = Solution::decode_at_index(s2, k2);
    println!("Result 2: {}", result2);  // Output: "h"

    let s3 = "a2345678999999999999999".to_string();
    let k3 = 1;
    let result3 = Solution::decode_at_index(s3, k3);
    println!("Result 3: {}", result3);  // Output: "a"
}
