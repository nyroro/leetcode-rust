
impl Solution {
    pub fn min_cost_set_time(start_at: i32, move_cost: i32, push_cost: i32, target_seconds: i32) -> i32 {
        let m = target_seconds / 60;
        let s = target_seconds % 60;

        let mut k = vec![m / 10, m % 10, s / 10, s % 10];

        fn cal(arr: &Vec<i32>, start_at: i32, move_cost: i32, push_cost: i32) -> i32 {
            let mut ans = 0;
            let mut flag = false;
            let mut s = start_at;
            for &t in arr {
                if t == 0 && !flag {
                    continue;
                }
                flag = true;
                if t != s {
                    s = t;
                    ans += move_cost;
                }
                ans += push_cost;
            }
            ans

        }

        let mut a = std::i32::MAX;
        if m <= 99 {
            a = a.min(cal(&k, start_at, move_cost, push_cost));
        }

        if s + 60 <= 99 && m >= 1 {
            let mut s = s + 60;
            let mut m = m - 1;
            k = vec![m / 10, m % 10, s / 10, s % 10];
            a = a.min(cal(&k, start_at, move_cost, push_cost));
        }
        a

    }
}
