
impl Solution {
    pub fn color_the_array(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut nums = vec![0; n as usize];
        let mut ret = Vec::new();
        let mut cnt = 0;

        for query in queries {
            let i = query[0] as usize;
            let c = query[1];

            if i > 0 && nums[i - 1] == nums[i] && nums[i] != 0 {
                cnt -= 1;
            }
            if i + 1 < nums.len() && nums[i + 1] == nums[i] && nums[i] != 0 {
                cnt -= 1;
            }
            nums[i] = c;
            if i > 0 && nums[i - 1] == nums[i] {
                cnt += 1;
            }
            if i + 1 < nums.len() && nums[i + 1] == nums[i] {
                cnt += 1;
            }
            ret.push(cnt);
        }

        ret

    }
}
