
impl Solution {
    pub fn construct_array(n: i32, mut k: i32) -> Vec<i32> {
        let mut answer: Vec<i32> = Vec::new();
        let mut left = 1;
        let mut right = n;

        while left <= right {
            if k > 1 {
                if k % 2 == 1 {
                    answer.push(left);
                    left += 1;
                } else {
                    answer.push(right);
                    right -= 1;
                }
                k -= 1;
            } else {
                answer.push(left);
                left += 1;
            }
        }

        answer

    }
}
