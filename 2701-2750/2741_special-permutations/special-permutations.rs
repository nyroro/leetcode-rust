


use std::collections::HashMap;

impl Solution {
    pub fn special_perm(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut mat = vec![vec![false; n]; n];

        for i in 0..n {
            for j in (i + 1)..n {
                if nums[i] % nums[j] == 0 || nums[j] % nums[i] == 0 {
                    mat[i][j] = true;
                    mat[j][i] = true;
                }
            }
        }

        let mut q: Vec<(usize, usize)> = Vec::new();
        let mut table: HashMap<(usize, usize), i32> = HashMap::new();
        let mod_num = 1_000_000_007;

        for (i, t) in nums.iter().enumerate() {
            q.push((i, 1 << i));
            table.insert((i, 1 << i), 1);
        }

        let mut qi = 0;
        while qi < q.len() {
            let (x, state) = q[qi];
            let v = *table.get(&(x, state)).unwrap_or(&0);
            qi += 1;
            for i in 0..n {
                if mat[x][i] && (state & (1 << i)) == 0 {
                    let nxt = i;
                    let nxt_state = state | (1 << i);
                    if !table.contains_key(&(nxt, nxt_state)) {
                        table.insert((nxt, nxt_state), 0);
                        q.push((nxt, nxt_state));
                    }
                    *table.get_mut(&(nxt, nxt_state)).unwrap() += v;
                    *table.get_mut(&(nxt, nxt_state)).unwrap() %= mod_num;
                }
            }
        }

        let mut ret = 0;
        for i in 0..n {
            ret = (ret + *table.get(&(i, (1 << n) - 1)).unwrap_or(&0)) % mod_num;
        }
        ret as i32

    }
}
