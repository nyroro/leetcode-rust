
struct ProductOfNumbers {
    nums: Vec<i32>,
}

impl ProductOfNumbers {
    fn new() -> Self {
        ProductOfNumbers {
            nums: Vec::new(),
        }
    }
    
    fn add(&mut self, num: i32) {
        self.nums.push(num);
    }
    
    fn get_product(&self, k: i32) -> i32 {
        let start = self.nums.len() - k as usize;
        let end = self.nums.len();
        self.nums[start..end].iter().product()
    }
}
