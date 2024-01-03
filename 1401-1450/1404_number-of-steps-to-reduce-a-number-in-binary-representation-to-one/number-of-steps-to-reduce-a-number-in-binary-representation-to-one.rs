
impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut l = s.len() - 1;
        let mut carry = 0;
        let mut count = 0;
        while l > 0 {
            if s.chars().nth(l).unwrap().to_digit(10).unwrap() + carry == 0 {
                carry = 0;
                count += 1;
            } else if s.chars().nth(l).unwrap().to_digit(10).unwrap() + carry == 2 {
                carry = 1;
                count += 1;
            } else {
                carry = 1;
                count += 2;
            }
            l -= 1;
        }
        if carry == 1 {
            count += 1;
        }
        count

    }
}
