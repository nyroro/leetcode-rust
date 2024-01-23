
impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        let mut d = vec![0; n as usize];
        
        for road in &roads {
            let u = road[0] as usize;
            let v = road[1] as usize;
            d[u] += 1;
            d[v] += 1;
        }
        
        d.sort();
        let mut ret = 0;
        for (i, &t) in (1..=n).zip(d.iter()) {
            ret += i as i64 * t as i64;
        }
        ret

    }
}
