
impl Solution {
    pub fn kth_palindrome(queries: Vec<i32>, int_length: i32) -> Vec<i64> {
        let mut answer: Vec<i64> = Vec::new();
        
        for &query in queries.iter() {
            let result = Solution::calculate_palindrome(query, int_length);
            answer.push(result);
        }
        
        answer

    }
    
    fn calculate_palindrome(query: i32, int_length: i32) -> i64 {
        if int_length == 1 {
            if query < 10 {
                return query as i64;
            } else {
                return -1;
            }
        }
        
        let s = 10_i64.pow(((int_length - 1) / 2) as u32);
        let t = 10_i64.pow(((int_length + 1) / 2) as u32);
        
        let mut x = s + query as i64 - 1;
        
        if x >= t {
            return -1;
        } else {
            if int_length % 2 == 0 {
                return x * t + Solution::reverse(x);
            } else {
                return x * t / 10 + Solution::reverse(x / 10);
            }
        }
    }
    
    fn reverse(mut num: i64) -> i64 {
        let mut rev = 0;
        while num > 0 {
            let digit = num % 10;
            rev = rev * 10 + digit;
            num /= 10;
        }
        rev

    }
}
