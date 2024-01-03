
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut ugly = vec![1];
        let (mut p2, mut p3, mut p5) = (0, 0, 0);
        
        for _ in 1..n {
            let next_ugly = *vec![
                ugly[p2] * 2,
                ugly[p3] * 3,
                ugly[p5] * 5

            ].iter().min().unwrap();
            
            ugly.push(next_ugly);
            
            if ugly[p2] * 2 == next_ugly {
                p2 += 1;
            }
            if ugly[p3] * 3 == next_ugly {
                p3 += 1;
            }
            if ugly[p5] * 5 == next_ugly {
                p5 += 1;
            }
        }
        
        ugly[n as usize - 1]
    }
}
