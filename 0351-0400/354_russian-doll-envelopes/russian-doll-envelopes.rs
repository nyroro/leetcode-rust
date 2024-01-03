
impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let mut envelopes = envelopes;
        envelopes.sort_by(|a, b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });
        
        let mut dp = Vec::new();
        
        for envelope in envelopes {
            let idx = match dp.binary_search(&envelope[1]) {
                Ok(idx) => idx,
                Err(idx) => idx,
            };
            if idx == dp.len() {
                dp.push(envelope[1]);
            } else {
                dp[idx] = envelope[1];
            }
        }
        
        dp.len() as i32

    }
}
