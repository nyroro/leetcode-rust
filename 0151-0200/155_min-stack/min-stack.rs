
struct MinStack {
    data: Vec<i32>,
    min: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            data: Vec::new(),
            min: Vec::new(),
        }
    }
    
    fn push(&mut self, val: i32) {
        self.data.push(val);
        if self.min.is_empty() || val <= *self.min.last().unwrap() {
            self.min.push(val);
        }
    }
    
    fn pop(&mut self) {
        if self.data.pop().unwrap() == *self.min.last().unwrap() {
            self.min.pop();
        }
    }
    
    fn top(&self) -> i32 {
        *self.data.last().unwrap()
    }
    
    fn get_min(&self) -> i32 {
        *self.min.last().unwrap()
    }
}
