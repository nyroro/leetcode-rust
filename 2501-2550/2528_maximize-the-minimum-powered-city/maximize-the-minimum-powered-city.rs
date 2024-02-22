


impl Solution {
    pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
        let n = stations.len() as usize;
        let r = r as usize;
        let k = k as i64;
        let mut power = vec![0; n + 1];

        for i in 0..n {
            let llim = i.saturating_sub(r);
            let rlim = (i + r).min(n - 1);
            power[llim] += stations[i] as i64;
            power[rlim + 1] -= stations[i] as i64;
        }

        for i in 1..=n {
            power[i] += power[i - 1];
        }

        let mut lo = 0;
        let mut hi = power[n - 1] + k;
        let mut ans = 0;

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            let mut flag = true;
            let mut pref = 0;
            let mut remain = k;
            let mut tmp = vec![0; n + 1];

            for i in 0..n {
                if power[i] + pref < mid {
                    let rem = mid - power[i] - pref;
                    if remain < rem {
                        flag = false;
                        break;
                    }
                    tmp[i] += rem;
                    tmp[(i + 2 * r).min(n)] -= rem;
                    remain -= rem;
                }
                pref += tmp[i];
            }

            if flag {
                ans = mid;
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }

        ans

    }
}
