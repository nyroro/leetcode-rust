
impl Solution {
    pub fn xor_game(nums: Vec<i32>) -> bool {
        // 如果数组长度为偶数，Alice必定获胜

        if nums.len() % 2 == 0 {
            return true;
        }
        
        let mut xor = 0;
        for num in &nums {
            xor ^= num;
        }
        
        // 如果数组中所有元素的异或结果为0，Alice获胜；否则，Alice失败

        xor == 0

    }
}
