
impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let mut result = Vec::new();
        if n == 1 {
            for i in 0..=9 {
                result.push(i);
            }
            return result;
        }
        for i in 1..=9 {
            Self::dfs(n - 1, k, i, &mut result);
        }
        result

    }
    
    fn dfs(n: i32, k: i32, num: i32, result: &mut Vec<i32>) {
        if n == 0 {
            result.push(num);
            return;
        }
        let last_digit = num % 10;
        if last_digit + k <= 9 {
            Self::dfs(n - 1, k, num * 10 + last_digit + k, result);
        }
        if k != 0 && last_digit - k >= 0 {
            Self::dfs(n - 1, k, num * 10 + last_digit - k, result);
        }
    }
}
