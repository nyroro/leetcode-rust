


impl Solution {
    pub fn largest_variance(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let unique_chars: Vec<char> = chars.iter().cloned().collect::<std::collections::HashSet<_>>().into_iter().collect();

        if unique_chars.len() == 1 {
            return 0;
        }

        let mut max_variance = 0;

        for i in 0..unique_chars.len() {
            for j in 0..unique_chars.len() {
                if i != j {
                    let x = unique_chars[i];
                    let y = unique_chars[j];
                    let mut now = 0;
                    let mut m = 1000000;
                    let mut buff = 0;

                    for &t in &chars {
                        if t == x {
                            now += 1;
                        } else if t == y {
                            now -= 1;
                            m = std::cmp::min(m, buff);
                            buff = now;
                        }
                        max_variance = std::cmp::max(max_variance, now - m);
                    }
                }
            }
        }

        max_variance

    }
}

fn main() {
    println!("{}", Solution::largest_variance(String::from("aababbb"))); // Output: 3

    println!("{}", Solution::largest_variance(String::from("abcde"))); // Output: 0

}
