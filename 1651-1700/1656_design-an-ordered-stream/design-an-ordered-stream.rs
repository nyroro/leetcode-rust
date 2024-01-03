
struct OrderedStream {
    values: Vec<Option<String>>,
    ptr: usize,
}

impl OrderedStream {
    fn new(n: i32) -> Self {
        Self {
            values: vec![None; n as usize],
            ptr: 0,
        }
    }
    
    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        let id = id_key as usize - 1;
        self.values[id] = Some(value);
        
        let mut result = Vec::new();
        while self.ptr < self.values.len() && self.values[self.ptr].is_some() {
            result.push(self.values[self.ptr].take().unwrap());
            self.ptr += 1;
        }
        
        result

    }
}
