
use rand::seq::SliceRandom;
use rand::thread_rng;

struct Solution {
    nums: Vec<i32>,
    original_nums: Vec<i32>,
}

impl Solution {
    /** 
     * `&self` 表示该方法接受一个不可变引用。
     * 如果需要可变引用，请将其改为 `&mut self`。
     */
    fn new(nums: Vec<i32>) -> Self {
        Solution {
            nums: nums.clone(),
            original_nums: nums,
        }
    }
    
    fn reset(&self) -> Vec<i32> {
        self.original_nums.clone()
    }
    
    fn shuffle(&mut self) -> Vec<i32> {
        let mut rng = thread_rng();
        self.nums.shuffle(&mut rng);
        self.nums.clone()
    }
}
