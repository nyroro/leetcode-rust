
impl Solution {
    pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort_by(|a, b| b.cmp(a)); // 对数组进行排序


        let total_sum: i32 = sorted_nums.iter().sum(); // 计算数组的总和

        let mut sub_sum = 0;
        let mut result = Vec::new();

        for num in sorted_nums {
            sub_sum += num;
            result.push(num);

            if sub_sum > total_sum - sub_sum {
                return result;
            }
        }

        Vec::new()
    }
}
