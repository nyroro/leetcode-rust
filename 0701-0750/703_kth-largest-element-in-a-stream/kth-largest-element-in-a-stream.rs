
struct KthLargest {
    k: usize,
    nums: Vec<i32>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        Self {
            k: k as usize,
            nums,
        }
    }
    
    fn add(&mut self, val: i32) -> i32 {
        self.nums.push(val);
        self.nums.sort_unstable_by(|a, b| b.cmp(a));
        if self.nums.len() > self.k {
            self.nums.truncate(self.k);
        }
        self.nums[self.k - 1]
    }
}
