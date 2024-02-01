


impl Solution {
    pub fn k_mirror(k: i32, n: i32) -> i64 {
        let mut ret: Vec<i64> = (1..k).collect();

        fn gao(x: i32, y: Option<i32>, k: i32, ret: &mut Vec<i64>) {
            let mut num = x;
            if let Some(y_val) = y {
                num = num * k + y_val;
            }
            let mut tx = x;
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
            gao(i, None, k, &mut ret);
        }
        while ret.len() < n as usize {
            for i in k.pow(x) .. k.pow(x + 1) {
                gao(i / k, Some(i % k), k, &mut ret);
                if ret.len() >= n as usize {
                    break;
                }
            }
            if ret.len() >= n as usize {
                break;
            }
            for i in k.pow(x) .. k.pow(x + 1) {
                gao(i, None, k, &mut ret);
                if ret.len() >= n as usize {
                    break;
                }
            }
            x += 1;
        }
        ret.iter().take(n as usize).sum()
    }
}
