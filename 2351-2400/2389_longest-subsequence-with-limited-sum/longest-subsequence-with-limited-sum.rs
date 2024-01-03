
impl Solution {
    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut answer = Vec::new();
        for &query in &queries {
            let mut sorted_nums = nums.clone();
            sorted_nums.sort_unstable();
            let mut sum = 0;
            let mut count = 0;
            for &num in sorted_nums.iter() {
                sum += num;
                if sum <= query {
                    count += 1;
                } else {
                    break;
                }
            }
            answer.push(count);
        }
        answer

    }
}
