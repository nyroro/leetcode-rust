
impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort(); // Sort the input array


        let mut banned_range = vec![];
        let n = nums.len();

        for i in 0..n {
            let mut l = 100;
            let mut r = -1;
            for j in 0..i {
                if nums[i] == nums[j] + k {
                    l = l.min(j as i32);
                    r = r.max(j as i32);
                }
            }
            banned_range.push((l, r));
        }

        let mut tot = 0;
        let mut l = vec![];

        fn dfs(l: &mut Vec<bool>, banned_range: &[(i32, i32)], tot: &mut i32, n: usize) {
            if n == l.len() {
                return;
            } else {
                if let Some((left, right)) = banned_range.get(l.len()) {
                    if !l.get(*left as usize..=*right as usize).unwrap_or(&[]).iter().any(|&x| x) {
                        *tot += 1;
                        let mut l_true = l.clone();
                        l_true.push(true);
                        dfs(&mut l_true, banned_range, tot, n);
                    }
                }
                let mut l_false = l.clone();
                l_false.push(false);
                dfs(&mut l_false, banned_range, tot, n);
            }
        }

        dfs(&mut l, &banned_range, &mut tot, n);
        tot

    }
}
