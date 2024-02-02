


impl Solution {
    pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
        let mut ans = vec![0];
        let mut m = k;
        loop {
            if !ans.contains(&((ans.last().unwrap() + m) % n)) {
                ans.push((ans.last().unwrap() + m) % n);
                m += k;
            } else {
                break;
            }
        }
        (1..=n).filter(|x| !ans.contains(&(x - 1))).collect()
    }
}
