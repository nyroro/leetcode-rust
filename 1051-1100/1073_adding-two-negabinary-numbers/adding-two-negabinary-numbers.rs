
impl Solution {
    pub fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut carry: i32 = 0;
        let (mut i, mut j) = (arr1.len() as i32 - 1, arr2.len() as i32 - 1);

        while i >= 0 || j >= 0 || carry != 0 {
            if i >= 0 {
                carry += arr1[i as usize];
                i -= 1;
            }
            if j >= 0 {
                carry += arr2[j as usize];
                j -= 1;
            }
            result.push(carry & 1);
            carry = -(carry >> 1);
        }

        while result.len() > 1 && *result.last().unwrap() == 0 {
            result.pop();
        }

        result.reverse();
        result

    }
}
