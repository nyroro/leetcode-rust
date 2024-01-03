
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        // Step 1: Get the XOR of the two unique elements

        let mut xor = 0;
        for num in &nums {
            xor ^= num;
        }
        
        // Step 2: Find a bit where the two elements differ

        let diff_bit = xor & (-xor);
        
        // Step 3: Divide the elements into two groups based on the diff_bit

        let mut group1 = 0;
        let mut group2 = 0;
        for num in &nums {
            if num & diff_bit != 0 {
                group1 ^= num;
            } else {
                group2 ^= num;
            }
        }
        
        // Step 4: Return the two unique elements

        vec![group1, group2]
    }
}
