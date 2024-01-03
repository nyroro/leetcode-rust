
impl Solution {
    pub fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut carry = k;
        let mut i = num.len() as i32 - 1;
        
        while i >= 0 || carry > 0 {
            let digit = if i >= 0 { num[i as usize] } else { 0 };
            let sum = digit + carry;
            result.push(sum % 10);
            carry = sum / 10;
            i -= 1;
        }
        
        result.reverse();
        result

    }
}
