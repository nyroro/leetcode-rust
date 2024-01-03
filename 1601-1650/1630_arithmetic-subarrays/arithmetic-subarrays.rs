
impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let mut answer = vec![];

        for i in 0..l.len() {
            let start = l[i] as usize;
            let end = r[i] as usize;

            let mut sub_array = nums[start..=end].to_vec();
            sub_array.sort();

            let diff = sub_array[1] - sub_array[0];
            let mut is_arithmetic = true;

            for j in 1..sub_array.len() {
                if sub_array[j] - sub_array[j - 1] != diff {
                    is_arithmetic = false;
                    break;
                }
            }

            answer.push(is_arithmetic);
        }

        answer

    }
}
