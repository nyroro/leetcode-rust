
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result = digits;
        let mut carry = 1;
        
        for i in (0..result.len()).rev() {
            if result[i] + carry >= 10 {
                result[i] = 0;
                carry = 1;
            } else {
                result[i] += carry;
                carry = 0;
            }
        }
        
        if carry == 1 {
            result.insert(0, 1);
        }
        
        result

    }
}
