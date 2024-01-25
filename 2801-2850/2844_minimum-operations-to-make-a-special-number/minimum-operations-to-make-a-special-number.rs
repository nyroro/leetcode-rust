impl Solution {
    pub fn minimum_operations(num: String) -> i32 {
        let mut dp = vec![1000000000; 25];
        dp[0] = 0;
        
        for t in num.chars() {
            let mut nxt = vec![1000000000; 25];
            let t = t.to_digit(10).unwrap() as usize;
            
            for i in 0..25 {
                nxt[i] = nxt[i].min(dp[i] + 1);
            }
            
            for i in 0..25 {
                let x = (i * 10 + t) % 25;
                nxt[x] = dp[i].min(nxt[x]);
            }
            
            dp = nxt;
        }
        
        dp[0]
    }
}
