
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let roman_nums: std::collections::HashMap<char, i32> = [
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]
        .iter()
        .cloned()
        .collect();

        let chars: Vec<char> = s.chars().collect();
        let mut total = 0;

        for i in 0..chars.len() {
            let current = roman_nums[&chars[i]];
            if i < chars.len() - 1 && current < roman_nums[&chars[i + 1]] {
                total -= current;
            } else {
                total += current;
            }
        }

        total

    }
}
