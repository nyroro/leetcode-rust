
impl Solution {
    pub fn min_wasted_space(packages: Vec<i32>, boxes: Vec<Vec<i32>>) -> i32 {
        const INF: i64 = 1e18 as i64;
        const MOD: i64 = 1e9 as i64 + 7;

        let mut packages = packages;
        packages.sort();

        let mut pre = Vec::new();
        let mut sum = 0;
        for &u in &packages {
            sum += u as i64;
            pre.push(sum);
        }

        let mut ans = INF;

        for mut b in boxes {
            b.sort();

            let mut idx = -1;
            let mut cur = 0;

            for &u in &b {
                let (mut l, mut r) = (idx, packages.len() as i32);
                while r - l > 1 {
                    let mid = (l + r) / 2;
                    if packages[mid as usize] > u {
                        r = mid;
                    } else {
                        l = mid;
                    }
                }

                if l == idx {
                    continue;
                }

                cur += (l - idx) as i64 * u as i64 - pre[l as usize] + if idx >= 0 { pre[idx as usize] } else { 0 };

                idx = l;
            }

            if idx == (packages.len() - 1) as i32 {
                ans = ans.min(cur);
            }
        }

        if ans == INF {
            -1

        } else {
            (ans % MOD) as i32

        }
    }
}
