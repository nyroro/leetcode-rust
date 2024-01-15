
impl Solution {
    pub fn divide_array(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort(); // 对数组进行排序

        let mut result = Vec::new();
        let mut current_array = Vec::new();
        for &num in nums.iter() {
            if current_array.len() == 0 {
                current_array.push(num);
            } else {
                if num - current_array[0] <= k && current_array.len() < 3 {
                    current_array.push(num);
                } else {
                    if current_array.len() == 3 {
                        result.push(current_array.clone());
                        current_array.clear();
                        current_array.push(num);
                    } else {
                        return Vec::new(); // 无法满足条件，返回空数组

                    }
                }
            }
        }
        if current_array.len() == 3 {
            result.push(current_array.clone());
        }
        result

    }
}
