


impl Solution {
    pub fn recover_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort(); // 对数组进行排序

        let x = nums[0]; // 获取最小值


        let mut ret = vec![0; nums.len() / 2]; // 创建用于存储结果的数组


        // 定义递归函数

        fn gao(nums: &mut Vec<i32>, ret: &mut Vec<i32>, x: i32) -> bool {
            let mut table = std::collections::HashMap::new();
            let mut ri = 0;
            for i in 0..nums.len() {
                if table.get(&nums[i]).cloned().unwrap_or(0) > 0 {
                    *table.get_mut(&nums[i]).unwrap() -= 1;
                } else if ri >= ret.len() {
                    return false;
                } else {
                    ret[ri] = nums[i] + x;
                    ri += 1;
                    *table.entry(nums[i] + 2 * x).or_insert(0) += 1;
                }
            }
            table.values().sum::<i32>() == 0

        }

        for &t in &nums {
            if t > x && (t - x) % 2 == 0 {
                let k = (t - x) / 2;
                if gao(&mut nums, &mut ret, k) {
                    return ret;
                }
            }
        }
        ret

    }
}
