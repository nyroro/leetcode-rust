
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut answer = Vec::new();
        
        for i in 1..=n {
            if i % 3 == 0 && i % 5 == 0 {
                answer.push("FizzBuzz".to_string());
            } else if i % 3 == 0 {
                answer.push("Fizz".to_string());
            } else if i % 5 == 0 {
                answer.push("Buzz".to_string());
            } else {
                answer.push(i.to_string());
            }
        }
        
        answer

    }
}
