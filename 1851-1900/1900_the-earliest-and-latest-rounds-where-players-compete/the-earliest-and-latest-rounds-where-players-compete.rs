
use std::collections::HashMap;



impl Solution {
    pub fn earliest_and_latest(n: i32, first_player: i32, second_player: i32) -> Vec<i32> {
        let a = (first_player - 1) as usize;
        let b = (second_player - first_player - 1) as usize;
        let c = (n - second_player) as usize;
        let mut ret = vec![1000, -1];
        let mut memo: HashMap<(usize, usize, usize, usize), ()> = HashMap::new();

        fn dfs(a: usize, b: usize, c: usize, x: i32, ret: &mut Vec<i32>, memo: &mut HashMap<(usize, usize, usize, usize), ()>) {
            if a == c {
                ret[0] = ret[0].min(x);
                ret[1] = ret[1].max(x);
                return;
            }
            let n = a + b + c + 2;
            let key = (a, b, c, x);
            if memo.contains_key(&key) {
                return;
            }
            memo.insert(key, ());

            if a < c {
                dfs(c, b, a, x, ret, memo);
            } else {
                if a < n / 2 {
                    for i in 0..=c {
                        let na = i;
                        let nc = c - i;
                        for j in 0..(a - c) {
                            let nna = na + j;
                            let nb = a - c - j - 1 + (b + c - a + 1) / 2;
                            dfs(nna, nb, nc, x + 1, ret, memo);
                        }
                    }
                } else {
                    for i in 0..=c {
                        let na = i;
                        let nc = c - i;
                        for j in 0..=b {
                            let nna = na + j + (a - c - b - 1) / 2;
                            let nb = b - j;
                            dfs(nna, nb, nc, x + 1, ret, memo);
                        }
                    }
                }
            }
        }

        dfs(a, b, c, 1, &mut ret, &mut memo);
        ret

    }
}
