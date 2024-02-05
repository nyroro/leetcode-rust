


impl Solution {
    pub fn cycle_length_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = Vec::new();
        
        for query in queries {
            let mut ans = 1;
            let mut x = query[0];
            let mut y = query[1];
            
            while x != y {
                if x > y {
                    x /= 2;
                } else {
                    y /= 2;
                }
                ans += 1;
            }
            
            res.push(ans);
        }
        
        res

    }
}
