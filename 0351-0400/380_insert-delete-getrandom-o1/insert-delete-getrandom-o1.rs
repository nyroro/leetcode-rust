
use std::collections::HashSet;
use rand::Rng;

struct RandomizedSet {
    set: HashSet<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            set: HashSet::new(),
        }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        self.set.insert(val)
    }
    
    fn remove(&mut self, val: i32) -> bool {
        self.set.remove(&val)
    }
    
    fn get_random(&self) -> i32 {
        let index = rand::thread_rng().gen_range(0, self.set.len());
        let random_element = self.set.iter().nth(index).unwrap();
        *random_element

    }
}
