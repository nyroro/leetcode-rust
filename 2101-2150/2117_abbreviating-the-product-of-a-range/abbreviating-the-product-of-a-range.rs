
impl Solution {
    pub fn abbreviate_product(left: i32, right: i32) -> String {
        let mut nums: Vec<i32> = (left..=right).collect();
        let mut t2 = 0;
        let mut t5 = 0;

        for t in &mut nums {
            let mut temp = *t;
            while temp % 2 == 0 {
                temp /= 2;
                t2 += 1;
            }
            while temp % 5 == 0 {
                temp /= 5;
                t5 += 1;
            }
            *t = temp;
        }

        let c = std::cmp::min(t2, t5);
        let mut t2 = t2 - c;
        let mut t5 = t5 - c;
        let mut ans: i64 = 1;
        let mut flag = false;

        for _ in 0..t2 {
            ans *= 2;
            if ans >= 10000000000 {
                flag = true;
            }
            ans %= 10000000000;
        }

        for _ in 0..t5 {
            ans *= 5;
            if ans >= 10000000000 {
                flag = true;
            }
            ans %= 10000000000;
        }

        for z in nums {
            ans *= z as i64;
            if ans >= 10000000000 {
                flag = true;
            }
            ans %= 10000000000;
        }

        let mut result = String::new();

        if !flag {
            result.push_str(&format!("{}e{}", ans, c));
        } else {
            let t = (left..=right).map(|t| t as f64).sum::<f64>().log10();
            let t = 4.0 + (t - t.trunc());
            let t = 10_f64.powf(t) as i64;
            let r = ans.to_string().chars().rev().take(5).collect::<String>().chars().rev().collect::<String>();
            result.push_str(&format!("{}...{}e{}", t, r, c));
        }

        result

    }
}
