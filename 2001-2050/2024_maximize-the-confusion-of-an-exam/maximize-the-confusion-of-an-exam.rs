
impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let answer: Vec<char> = answer_key.chars().collect();
        let n = answer.len();
        let mut left = 0;
        let mut right = 0;
        let mut max_len = 0;
        let mut max_count = 0;
        let mut count = vec![0; 2];

        while right < n {
            count[(answer[right] == 'T') as usize] += 1;
            max_count = max_count.max(count[(answer[right] == 'T') as usize]);

            if (right - left + 1) - max_count > k as usize {
                count[(answer[left] == 'T') as usize] -= 1;
                left += 1;
            }

            max_len = max_len.max(right - left + 1);
            right += 1;
        }

        max_len as i32

    }
}
