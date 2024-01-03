
impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut stack = Vec::new();
        let mut k = k;

        for digit in num.chars() {
            while k > 0 && !stack.is_empty() && stack[stack.len() - 1] > digit {
                stack.pop();
                k -= 1;
            }
            stack.push(digit);
        }

        // Handle the case where k is still greater than 0

        while k > 0 {
            stack.pop();
            k -= 1;
        }

        // Construct the final result string

        let mut result = String::new();
        let mut leading_zero = true;
        for digit in stack {
            if leading_zero && digit == '0' {
                continue;
            }
            leading_zero = false;
            result.push(digit);
        }

        // If the result is empty, return "0"
        if result.is_empty() {
            return "0".to_string();
        }

        result

    }
}
