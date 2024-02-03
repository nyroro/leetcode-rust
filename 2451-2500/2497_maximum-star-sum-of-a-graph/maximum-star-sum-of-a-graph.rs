


impl Solution {
    pub fn max_star_sum(vals: Vec<i32>, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        if k == 0 {
            return *vals.iter().max().unwrap();
        }
        
        let n = vals.len();
        let mut answ = *vals.iter().max().unwrap();
        let mut stars: Vec<Vec<i32>> = vec![Vec::new(); n];
        
        for edge in &edges {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            if vals[b] > 0 {
                stars[a].push(vals[b]);
            }
            if vals[a] > 0 {
                stars[b].push(vals[a]);
            }
        }
        
        for i in 0..n {
            let mut star_vals = stars[i].clone();
            star_vals.sort_unstable();
            star_vals.reverse();
            let sum: i32 = vals[i] + star_vals.iter().take(k as usize).sum::<i32>();
            answ = answ.max(sum);
        }
        
        answ

    }
}
