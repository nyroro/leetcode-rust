
impl Solution {
    pub fn min_deletion(nums: Vec<i32>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        let mut deletions = 0;

        for &num in nums.iter() {
            if stack.is_empty() {
                stack.push(num);
            } else if stack.len() % 2 == 1 {
                if stack[stack.len() - 1] == num {
                    deletions += 1;
                } else {
                    stack.push(num);
                }
            } else {
                stack.push(num);
            }
        }

        if stack.len() % 2 != 0 {
            deletions += 1;
        }

        deletions

    }
}
