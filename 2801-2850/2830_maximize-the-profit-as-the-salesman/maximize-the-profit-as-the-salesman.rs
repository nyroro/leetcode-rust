
struct Offer {
    start: i32,
    end: i32,
    gold: i32,
}

impl Solution {
    pub fn maximize_the_profit(n: i32, offers: Vec<Vec<i32>>) -> i32 {
        let mut groups: Vec<Vec<(i32, i32)>> = vec![vec![]; n as usize];
        
        for offer in &offers {
            let start = offer[0];
            let end = offer[1];
            let gold = offer[2];
            groups[end as usize].push((start, gold));
        }
        
        let mut f: Vec<i32> = vec![0; (n+1) as usize];
        
        for (end, x) in groups.iter().enumerate() {
            f[end+1] = f[end];
            for &(start, gold) in x {
                f[end+1] = f[end+1].max(f[start as usize] + gold);
            }
        }
        
        return f[n as usize];
    }
}
