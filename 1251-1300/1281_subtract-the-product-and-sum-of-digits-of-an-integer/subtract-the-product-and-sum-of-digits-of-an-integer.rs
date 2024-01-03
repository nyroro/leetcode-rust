
impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let digits = n.to_string();
        let mut product = 1;
        let mut sum = 0;

        for digit in digits.chars() {
            let num = digit.to_digit(10).unwrap() as i32;
            product *= num;
            sum += num;
        }

        product - sum

    }
}
