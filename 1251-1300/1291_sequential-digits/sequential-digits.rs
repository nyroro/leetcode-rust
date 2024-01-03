
impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let digits = "123456789";
        let mut result = Vec::new();

        for length in 1..=digits.len() {
            for start in 0..=digits.len() - length {
                let substring = &digits[start..start + length];
                let num = substring.parse::<i32>().unwrap();
                if num >= low && num <= high {
                    result.push(num);
                }
            }
        }

        result.sort();
        result

    }
}
