
const MOD: i64 = 1e9 as i64 + 7;

fn prime_factors_count(x: i32) -> i32 {
    let mut count = 0;
    let mut num = x as i64;
    let mut i = 2;
    while i * i <= num {
        let mut flag = false;
        while num % i == 0 {
            num /= i;
            if !flag {
                count += 1;
            }
            flag = true;
        }
        i += 1;
    }
    if num > 1 {
        count += 1;
    }
    count

}

fn expo(mut x: i64, mut y: i64) -> i64 {
    let mut res = 1;
    while y > 0 {
        if y & 1 == 1 {
            res = (res * x) % MOD;
        }
        x = (x * x) % MOD;
        y >>= 1;
    }
    res

}

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, mut k: i32) -> i32 {
        let n = nums.len();
        let mut dis_primes: Vec<i32> = nums.iter().map(|&x| prime_factors_count(x)).collect();
        let mut idx: Vec<usize> = (0..n).collect();
        idx.sort_by(|&i, &j| nums[j].cmp(&nums[i]));

        let mut vl: Vec<i32> = vec![0; n];
        let mut vr: Vec<i32> = vec![0; n];
        let mut st: Vec<usize> = Vec::new();

        for i in 0..n {
            while !st.is_empty() && dis_primes[*st.last().unwrap()] < dis_primes[i] {
                vr[st.pop().unwrap()] = i as i32;
            }
            st.push(i);
        }
        while !st.is_empty() {
            vr[st.pop().unwrap()] = n as i32;
        }

        for i in (0..n).rev() {
            while !st.is_empty() && dis_primes[*st.last().unwrap()] <= dis_primes[i] {
                vl[st.pop().unwrap()] = i as i32;
            }
            st.push(i);
        }
        while !st.is_empty() {
            vl[st.pop().unwrap()] = -1;
        }

        let mut ans: i64 = 1;
        for i in 0..n {
            let j = idx[i];
            let m = (vr[j] - vl[j] - 1) as i64;
            let val = ((j as i64 - vl[j] as i64) * (m - (j as i64 - vl[j] as i64 - 1))) as i64;
            if val <= k as i64 {
                ans = (ans * expo(nums[j] as i64, val) % MOD) % MOD;
                k -= val as i32;
            } else {
                ans = (ans * expo(nums[j] as i64, k as i64) % MOD) % MOD;
                break;
            }
        }
        ans as i32

    }
}
