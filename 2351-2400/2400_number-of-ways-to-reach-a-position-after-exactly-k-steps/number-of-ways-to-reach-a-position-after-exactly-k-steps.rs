


impl Solution {
    pub fn number_of_ways(start_pos: i32, end_pos: i32, k: i32) -> i32 {
        let modulo: i64 = 1_000_000_007;
        
        let ncr = |n: i32, r: i32| -> i64 {
            let mut v = vec![0; (n + 1) as usize];
            v[0] = 1;
            for i in 1..=n {
                for j in (1..=r).rev() {
                    v[j as usize] = ((v[j as usize] % modulo) + (v[(j - 1) as usize] % modulo)) % modulo;
                }
            }
            v[r as usize]
        };

        let diff = (start_pos - end_pos).abs();
        if diff > k || (diff + k) % 2 != 0 {
            return 0;
        }
        let r = (diff + k) / 2;
        ncr(k, r) as i32

    }
}
