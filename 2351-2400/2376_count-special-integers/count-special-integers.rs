
impl Solution {
    pub fn count_special_numbers(n: i32) -> i32 {
        let vals: Vec<i32> = n.to_string().chars().map(|c| c.to_digit(10).unwrap() as i32).collect();

        fn fn_count_special(i: usize, m: i32, on: bool, vals: &Vec<i32>) -> i32 {
            let mut ans = 0;
            if i == vals.len() {
                return 1;
            }
            for v in 0..if on { vals[i] } else { 10 } {
                if m & (1 << v) == 0 {
                    if m != 0 || v != 0 {
                        ans += fn_count_special(i + 1, m ^ (1 << v), false, &vals);
                    } else {
                        ans += fn_count_special(i + 1, m, false, &vals);
                    }
                }
            }
            if on && m & (1 << vals[i]) == 0 {
                ans += fn_count_special(i + 1, m ^ (1 << vals[i]), true, &vals);
            }
            ans

        }

        fn_count_special(0, 0, true, &vals) - 1

    }
}
