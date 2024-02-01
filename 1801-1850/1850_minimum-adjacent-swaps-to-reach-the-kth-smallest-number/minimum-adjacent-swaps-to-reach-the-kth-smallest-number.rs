
impl Solution {
    pub fn get_min_swaps(num: String, k: i32) -> i32 {
        let mut num = num.chars().collect::<Vec<char>>();
        let mut original = num.clone();
        
        // 辅助函数，用于计算下一个排列

        fn next_permutation(nums: &mut Vec<char>) {
            let mut i = nums.len() - 2;
            while i >= 0 && nums[i] >= nums[i + 1] {
                i -= 1;
            }
            if i >= 0 {
                let mut j = nums.len() - 1;
                while j > i && nums[j] <= nums[i] {
                    j -= 1;
                }
                nums.swap(i, j);
            }
            nums[i + 1..].reverse();
        }
        
        // 模拟得到第 k 个最小的 wonderful integer

        for _ in 0..k {
            next_permutation(&mut num);
        }
        
        // 计算原始数字和目标数字之间的最小交换次数

        let mut swaps = 0;
        for i in 0..num.len() {
            if num[i] != original[i] {
                let mut j = i + 1;
                while original[j] != num[i] {
                    j += 1;
                }
                while j > i {
                    original.swap(j, j - 1);
                    j -= 1;
                    swaps += 1;
                }
            }
        }
        
        swaps

    }
}
