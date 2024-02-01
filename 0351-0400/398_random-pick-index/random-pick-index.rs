
use rand::seq::SliceRandom;

struct Solution {
    nums: Vec<i32>,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Solution { nums }
    }

    fn pick(&self, target: i32) -> i32 {
        let mut indices: Vec<i32> = Vec::new();

        for (i, &num) in self.nums.iter().enumerate() {
            if num == target {
                indices.push(i as i32);
            }
        }

        let mut rng = rand::thread_rng();
        *indices.choose(&mut rng).unwrap_or(&-1)
    }
}
