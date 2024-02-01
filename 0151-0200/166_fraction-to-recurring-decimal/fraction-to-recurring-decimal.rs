
impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let mut result = String::new();
        if numerator == 0 {
            return "0".to_string();
        }
        if (numerator < 0) ^ (denominator < 0) {
            result.push('-');
        }
        let (mut numerator, mut denominator) = (numerator.abs() as i64, denominator.abs() as i64);
        let integer_part = numerator / denominator;
        result += &integer_part.to_string();
        let mut remainder = numerator % denominator;
        if remainder == 0 {
            return result;
        }
        result.push('.');
        let mut map = std::collections::HashMap::new();
        while remainder != 0 {
            if let Some(&pos) = map.get(&remainder) {
                result.insert(pos, '(');
                result.push(')');
                return result;
            }
            map.insert(remainder, result.len());
            remainder *= 10;
            result += &(remainder / denominator).to_string();
            remainder %= denominator;
        }
        result

    }
}
