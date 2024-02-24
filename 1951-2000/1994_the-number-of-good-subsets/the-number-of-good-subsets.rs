
use std::collections::{HashMap, HashSet};

impl Solution {
    fn power(x: i64, y: i32, modulo: i64) -> i64 {
        let mut res = 1;
        let mut x_mut = x % modulo;
        if x_mut == 0 {
            return 0;
        }
        let mut y_mut = y;
        while y_mut > 0 {
            if y_mut & 1 == 1 {
                res = (res * x_mut) % modulo;
            }
            y_mut >>= 1;
            x_mut = (x_mut * x_mut) % modulo;
        }
        res % modulo

    }

    fn solve(f: &Vec<i32>, i: i32, mask: i32, dp: &mut Vec<Vec<i64>>, good: &HashMap<i32, i32>, div: &HashMap<i32, Vec<i32>>, bad: &HashSet<i32>, modulo: i64) -> i64 {
        if i == 31 {
            if mask > 0 {
                return 1;
            } else {
                return 0;
            }
        }
        if dp[i as usize][mask as usize] != -1 {
            return dp[i as usize][mask as usize];
        }
        if f[i as usize] == 0 || bad.contains(&i) {
            return Solution::solve(f, i + 1, mask, dp, good, div, bad, modulo);
        } else if good.contains_key(&i) {
            if mask & (1 << good[&i]) > 0 {
                return Solution::solve(f, i + 1, mask, dp, good, div, bad, modulo);
            } else {
                let take = (f[i as usize] as i64 * Solution::solve(f, i + 1, mask | (1 << good[&i]), dp, good, div, bad, modulo)) % modulo;
                let not_take = Solution::solve(f, i + 1, mask, dp, good, div, bad, modulo);
                return (take + not_take) % modulo;
            }
        } else {
            let mut flag = true;
            let mut new_mask = mask;
            if let Some(divisors) = div.get(&i) {
                for &k in divisors {
                    new_mask |= 1 << good[&k];
                    if mask & (1 << good[&k]) > 0 {
                        flag = false;
                        break;
                    }
                }
            }
            if flag {
                let take = (f[i as usize] as i64 * Solution::solve(f, i + 1, new_mask, dp, good, div, bad, modulo)) % modulo;
                let not_take = Solution::solve(f, i + 1, mask, dp, good, div, bad, modulo);
                return (take + not_take) % modulo;
            } else {
                return Solution::solve(f, i + 1, mask, dp, good, div, bad, modulo);
            }
        }
    }

    pub fn number_of_good_subsets(nums: Vec<i32>) -> i32 {
        let modulo = 1e9 as i64 + 7;
        let mut good = HashMap::new();
        good.insert(2, 0);
        good.insert(3, 1);
        good.insert(5, 2);
        good.insert(7, 3);
        good.insert(11, 4);
        good.insert(13, 5);
        good.insert(17, 6);
        good.insert(19, 7);
        good.insert(23, 8);
        good.insert(29, 9);

        let mut div = HashMap::new();
        div.insert(6, vec![2, 3]);
        div.insert(10, vec![2, 5]);
        div.insert(14, vec![2, 7]);
        div.insert(15, vec![3, 5]);
        div.insert(21, vec![3, 7]);
        div.insert(22, vec![2, 11]);
        div.insert(26, vec![2, 13]);
        div.insert(30, vec![2, 3, 5]);

        let mut bad = HashSet::new();
        bad.insert(1);
        bad.insert(4);
        bad.insert(8);
        bad.insert(9);
        bad.insert(12);
        bad.insert(16);
        bad.insert(18);
        bad.insert(20);
        bad.insert(24);
        bad.insert(25);
        bad.insert(27);
        bad.insert(28);

        let mut dp = vec![vec![-1; 1 << 10]; 31];
        let mut f = vec![0; 31];
        for &i in &nums {
            f[i as usize] += 1;
        }

        let ans = (Solution::power(2, f[1], modulo) * Solution::solve(&f, 2, 0, &mut dp, &good, &div, &bad, modulo)) % modulo;
        ans as i32

    }
}
