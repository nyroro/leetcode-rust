
impl Solution {
    pub fn last_visited_integers(words: Vec<String>) -> Vec<i32> {
        let mut nums: Vec<i32> = Vec::new();
        let mut result: Vec<i32> = Vec::new();
        let mut prev_count = 0;

        for word in words.iter() {
            if word == "prev" {
                if prev_count < nums.len() {
                    result.push(nums[nums.len() - 1 - prev_count]);
                } else {
                    result.push(-1);
                }
                prev_count += 1;
            } else {
                let num = word.parse::<i32>().unwrap();
                nums.push(num);
                prev_count = 0;
            }
        }
        result

    }
}
