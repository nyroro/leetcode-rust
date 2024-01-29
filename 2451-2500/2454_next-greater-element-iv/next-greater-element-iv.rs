


impl Solution {
    pub fn second_greater_element(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![-1; nums.len()];
        let (mut s, mut ss): (Vec<usize>, Vec<usize>) = (Vec::new(), Vec::new());

        for (i, &x) in nums.iter().enumerate() {
            while let Some(top) = ss.last().cloned() {
                if nums[top] < x {
                    ans[ss.pop().unwrap()] = x;
                } else {
                    break;
                }
            }

            let mut buff = Vec::new();
            while let Some(top) = s.last().cloned() {
                if nums[top] < x {
                    buff.push(s.pop().unwrap());
                } else {
                    break;
                }
            }

            while let Some(item) = buff.pop() {
                ss.push(item);
            }

            s.push(i);
        }

        ans

    }
}
