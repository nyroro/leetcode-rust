
struct DataStream {
    value: i32,
    k: i32,
    stream: Vec<i32>,
}

impl DataStream {
    fn new(value: i32, k: i32) -> Self {
        DataStream {
            value,
            k,
            stream: Vec::new(),
        }
    }
    
    fn consec(&self, num: i32) -> bool {
        self.stream.push(num);
        if self.stream.len() < self.k as usize {
            return false;
        }
        for i in (self.stream.len() - self.k as usize)..self.stream.len() {
            if self.stream[i] != self.value {
                return false;
            }
        }
        true

    }
}
