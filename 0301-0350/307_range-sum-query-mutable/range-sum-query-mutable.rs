
struct NumArray {
    nums: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        NumArray { nums }
    }
    
    fn update(&mut self, index: i32, val: i32) {
        let index = index as usize;
        self.nums[index] = val;
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let left = left as usize;
        let right = right as usize;
        self.nums[left..=right].iter().sum()
    }
}
