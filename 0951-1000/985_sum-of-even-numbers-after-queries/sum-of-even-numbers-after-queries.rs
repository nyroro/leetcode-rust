
impl Solution {
    pub fn sum_even_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut answer = Vec::new();
        let mut sum = nums.iter().filter(|&num| num % 2 == 0).sum();

        for query in queries {
            let val_i = query[0];
            let index_i = query[1] as usize;

            if nums[index_i] % 2 == 0 {
                sum -= nums[index_i];
            }

            nums[index_i] += val_i;

            if nums[index_i] % 2 == 0 {
                sum += nums[index_i];
            }

            answer.push(sum);
        }

        answer

    }
}
