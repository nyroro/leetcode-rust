
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let n1 = num1.chars().rev().collect::<Vec<char>>();
        let n2 = num2.chars().rev().collect::<Vec<char>>();
        let mut result = vec![0; n1.len() + n2.len()];

        for i in 0..n1.len() {
            for j in 0..n2.len() {
                let digit1 = n1[i].to_digit(10).unwrap();
                let digit2 = n2[j].to_digit(10).unwrap();
                let product = digit1 * digit2;
                let sum = product + result[i + j];
                result[i + j] = sum % 10;
                result[i + j + 1] += sum / 10;
            }
        }

        while result.len() > 1 && result.last() == Some(&0) {
            result.pop();
        }

        result.iter().rev().map(|&digit| char::from_digit(digit, 10).unwrap()).collect()
    }
}
