
impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut sum = a;
        let mut carry = b;
        
        while carry != 0 {
            let temp = sum;
            sum = temp ^ carry;
            carry = (temp & carry) << 1;
        }
        
        sum

    }
}
