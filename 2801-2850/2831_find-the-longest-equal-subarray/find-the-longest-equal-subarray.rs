


impl Solution {
    pub fn longest_equal_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut cs: Vec<Vec<usize>> = vec![vec![]; n + 1];

        for (i, &c) in nums.iter().enumerate() {
            cs[c as usize].push(i);
        }

        let mut ans = 0;
        for i in 1..=n {
            let mut st = 0;
            let m = cs[i].len();
            for ed in 0..m {
                while st < ed && cs[i][ed] - cs[i][st] + 1 - (ed - st + 1) > k as usize {
                    st += 1;
                }
                ans = ans.max(ed - st + 1);
            }
        }
        ans as i32

    }
}
