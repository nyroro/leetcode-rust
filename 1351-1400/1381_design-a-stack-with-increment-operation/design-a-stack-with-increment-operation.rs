
struct CustomStack {
    stack: Vec<i32>,
    max_size: usize,
}

impl CustomStack {
    fn new(maxSize: i32) -> Self {
        Self {
            stack: Vec::new(),
            max_size: maxSize as usize,
        }
    }
    
    fn push(&mut self, x: i32) {
        if self.stack.len() < self.max_size {
            self.stack.push(x);
        }
    }
    
    fn pop(&mut self) -> i32 {
        self.stack.pop().unwrap_or(-1)
    }
    
    fn increment(&mut self, k: i32, val: i32) {
        let k = k as usize;
        let n = self.stack.len();
        for i in 0..k.min(n) {
            self.stack[i] += val;
        }
    }
}
