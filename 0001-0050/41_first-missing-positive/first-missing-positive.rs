
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        
        // 将所有正整数放到它们应该在的位置上

        for i in 0..n {
            let mut num = nums[i];
            while num > 0 && num <= n as i32 && nums[(num - 1) as usize] != num {
                let temp = nums[(num - 1) as usize];
                nums[(num - 1) as usize] = num;
                num = temp;
            }
        }
        
        // 找到第一个不在正确位置上的数字

        for i in 0..n {
            if nums[i] != i as i32 + 1 {
                return i as i32 + 1;
            }
        }
        
        // 如果数组中的所有数字都在正确位置上，则返回n+1

        return n as i32 + 1;
    }
}
