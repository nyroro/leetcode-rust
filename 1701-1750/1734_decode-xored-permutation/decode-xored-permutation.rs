
impl Solution {
    pub fn decode(encoded: Vec<i32>) -> Vec<i32> {
        let n = encoded.len() + 1;
        let mut perm: Vec<i32> = vec![0; n];
        
        // Find the XOR of all elements from 1 to n

        let mut xor_all = 0;
        for i in 1..=n as i32 {
            xor_all ^= i;
        }
        
        // Find the XOR of all even-indexed elements in the encoded array

        let mut xor_even = 0;
        for i in (1..n - 1).step_by(2) {
            xor_even ^= encoded[i];
        }
        
        // Calculate the first element of the perm array

        perm[0] = xor_all ^ xor_even;
        
        // Calculate the rest of the perm array based on the first element

        for i in 1..n {
            perm[i] = perm[i - 1] ^ encoded[i - 1];
        }
        
        perm

    }
}
