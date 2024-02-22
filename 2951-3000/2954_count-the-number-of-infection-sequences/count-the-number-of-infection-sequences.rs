


impl Solution {
    const MOD: i64 = 1_000_000_007;

    fn pre() -> Vec<i64> {
        let mut fact = vec![0; 1e5 as usize + 1];
        fact[0] = 1;
        fact[1] = 1;
        for i in 2..=1e5 as usize {
            fact[i] = fact[i - 1] * i as i64 % Self::MOD;
        }
        fact

    }

    fn power(n: i64, x: i64) -> i64 {
        if x <= 0 {
            return 1;
        }
        let cnt = Self::power(n, x / 2);
        if x % 2 == 1 {
            (cnt * cnt % Self::MOD * n) % Self::MOD

        } else {
            (cnt * cnt) % Self::MOD

        }
    }

    fn inverse(x: i64) -> i64 {
        Self::power(x, Self::MOD - 2)
    }

    fn ncr(n: i64, r: i64, fact: &Vec<i64>) -> i64 {
        (fact[n as usize] * Self::inverse(fact[r as usize]) % Self::MOD * Self::inverse(fact[(n - r) as usize]) % Self::MOD) % Self::MOD

    }

    pub fn number_of_sequence(n: i32, sick: Vec<i32>) -> i32 {
        let fact = Self::pre();
        let mut ans = 1;
        let mut tot_empty = 0;
        for i in 0..sick.len() {
            let empty = if i != 0 { sick[i] - sick[i - 1] - 1 } else { sick[i] };
            tot_empty += empty;
            if i != 0 {
                ans *= Self::power(2, empty as i64 - 1);
                ans %= Self::MOD;
            }
            ans *= Self::ncr(tot_empty as i64, empty as i64, &fact);
            ans %= Self::MOD;
        }
        let empty = n - 1 - *sick.last().unwrap();
        tot_empty += empty;
        ans *= Self::ncr(tot_empty as i64, empty as i64, &fact);
        ans %= Self::MOD;
        ans as i32

    }
}
