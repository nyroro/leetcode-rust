


impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut remaining_gifts = gifts.clone();
        for _ in 0..k {
            let max_index = remaining_gifts.iter().position(|&x| x == *remaining_gifts.iter().max().unwrap()).unwrap();
            let sqrt = (remaining_gifts[max_index] as f64).sqrt() as i32;
            remaining_gifts[max_index] = sqrt;
        }
        remaining_gifts.iter().map(|&x| x as i64).sum()
    }
}
