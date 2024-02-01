
impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let mut result = Vec::new();
        let mut expr = String::new();
        let num_bytes = num.as_bytes();
        Solution::backtrack(&num_bytes, target, 0, 0, 0, &mut expr, &mut result);
        result

    }

    fn backtrack(num: &[u8], target: i32, pos: usize, eval: i64, prev: i64, expr: &mut String, result: &mut Vec<String>) {
        if pos == num.len() {
            if eval == target as i64 {
                result.push(expr.clone());
            }
            return;
        }

        let mut value = 0;
        let len = expr.len();

        for i in pos..num.len() {
            if i != pos && num[pos] == b'0' {
                break;
            }

            value = value * 10 + (num[i] - b'0') as i64;
            let value_str = std::str::from_utf8(&num[pos..=i]).unwrap();

            if pos == 0 {
                Solution::backtrack(num, target, i + 1, value, value, &mut format!("{}{}", expr, value_str), result);
            } else {
                Solution::backtrack(num, target, i + 1, eval + value, value, &mut format!("{}+{}", expr, value_str), result);
                Solution::backtrack(num, target, i + 1, eval - value, -value, &mut format!("{}-{}", expr, value_str), result);
                Solution::backtrack(num, target, i + 1, eval - prev + prev * value, prev * value, &mut format!("{}*{}", expr, value_str), result);
            }
        }
    }
}
