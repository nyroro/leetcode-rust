
struct RandomizedCollection {
    elements: std::collections::HashMap<i32, usize>,
    values: Vec<i32>,
}

impl RandomizedCollection {
    fn new() -> Self {
        RandomizedCollection {
            elements: std::collections::HashMap::new(),
            values: Vec::new(),
        }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        let count = self.elements.entry(val).or_insert(0);
        *count += 1;
        self.values.push(val);
        *count == 1

    }
    
    fn remove(&mut self, val: i32) -> bool {
        if let Some(count) = self.elements.get_mut(&val) {
            if *count > 0 {
                *count -= 1;
                if let Some(pos) = self.values.iter().rposition(|&x| x == val) {
                    self.values.swap_remove(pos);
                }
                return true;
            }
        }
        false

    }
    
    fn get_random(&self) -> i32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        self.values[rng.gen_range(0, self.values.len())]
    }
}
