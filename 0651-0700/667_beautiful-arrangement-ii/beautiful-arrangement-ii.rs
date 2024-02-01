
impl Solution {
    pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
        let mut answer: Vec<i32> = Vec::new();
        let mut left = 1;
        let mut right = n;
        let mut flag = true;

        while left <= right {
            if k > 1 {
                if flag {
                    answer.push(left);
                    left += 1;
                } else {
                    answer.push(right);
                    right -= 1;
                }
                k -= 1;
                flag = !flag;
            } else {
                answer.push(left);
                left += 1;
            }
        }

        answer

    }
}
