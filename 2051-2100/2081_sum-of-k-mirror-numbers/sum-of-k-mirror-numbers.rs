


impl Solution {
    pub fn k_mirror(k: i32, n: i32) -> i64 {
        let mut ret: Vec<i64> = (1..k as i64).collect();

        fn gao(x: i64, y: Option<i64>, k: i64, ret: &mut Vec<i64>) {
            let mut num = x;
            if let Some(y_val) = y {
                num = num * k + y_val;
            }
            let mut x_val = x;
            while x_val > 0 {
                num = num * k + x_val % k;
                x_val /= k;
            }
            let s = num.to_string();
            if s.len() == 1 {
                ret.push(num);
            } else {
                let t = s.len() / 2;
                if &s[..t] == s.chars().rev().take(t).collect::<String>() {
                    ret.push(num);
                }
            }
        }

        let mut x = 1;
        for i in 1..k {
            gao(i as i64, None, k as i64, &mut ret);
        }
        while ret.len() < n as usize {
            for i in k.pow(x as u32) .. k.pow(x as u32 + 1) {
                gao((i / k).into(), Some((i % k).into()), k as i64, &mut ret);
                if ret.len() >= n as usize {
                    break;
                }
            }
            if ret.len() >= n as usize {
                break;
            }
            for i in k.pow(x as u32) .. k.pow(x as u32 + 1) {
                gao(i as i64, None, k as i64, &mut ret);
                if ret.len() >= n as usize {
                    break;
                }
            }
            x += 1;
        }
        ret.iter().take(n as usize).sum()
    }
}
