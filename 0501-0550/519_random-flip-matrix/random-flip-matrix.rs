
use rand::Rng;

struct Solution {
    m: i32,
    n: i32,
    arr: Vec<i32>,
}

impl Solution {
    fn new(m: i32, n: i32) -> Self {
        Solution {
            m,
            n,
            arr: vec![0; (m * n) as usize],
        }
    }
    
    fn flip(&mut self) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let mut x = rng.gen_range(0, self.m * self.n);
        while self.arr[x as usize] == 1 {
            x = rng.gen_range(0, self.m * self.n);
        }
        self.arr[x as usize] = 1;
        vec![x / self.n, x % self.n]
    }
    
    fn reset(&mut self) {
        self.arr = vec![0; (self.m * self.n) as usize];
    }
}
