
impl Solution {
    pub fn judge_point24(cards: Vec<i32>) -> bool {
        // 递归函数，用于生成所有可能的排列方式

        fn backtrack(nums: &mut Vec<f64>) -> bool {
            if nums.len() == 1 {
                // 如果只剩下一个数字，判断是否等于24

                return (nums[0] - 24.0).abs() < 1e-5;
            }
            let n = nums.len();
            for i in 0..n {
                for j in 0..n {
                    if i != j {
                        // 选取两个数字进行运算

                        let a = nums[i];
                        let b = nums[j];
                        // 生成新的数字列表

                        let mut new_nums = Vec::new();
                        for k in 0..n {
                            if k != i && k != j {
                                new_nums.push(nums[k]);
                            }
                        }
                        // 尝试四种运算方式

                        if backtrack(&mut [a + b].iter().chain(new_nums.iter()).cloned().collect()) ||
                            backtrack(&mut [a - b].iter().chain(new_nums.iter()).cloned().collect()) ||
                            backtrack(&mut [a * b].iter().chain(new_nums.iter()).cloned().collect()) ||
                            (b != 0.0 && backtrack(&mut [a / b].iter().chain(new_nums.iter()).cloned().collect())) {
                            return true;
                        }
                    }
                }
            }
            false

        }
        
        // 将整数数组转换为浮点数数组

        let mut nums = cards.iter().map(|&x| x as f64).collect();
        // 调用递归函数判断是否存在解

        backtrack(&mut nums)
    }
}
