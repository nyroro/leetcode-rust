
use std::collections::{HashMap, BTreeSet};

struct NumberContainers {
    index_to_number: HashMap<i32, i32>,
    number_to_index: HashMap<i32, i32>,
    numbers: BTreeSet<i32>,
}

impl NumberContainers {
    fn new() -> Self {
        NumberContainers {
            index_to_number: HashMap::new(),
            number_to_index: HashMap::new(),
            numbers: BTreeSet::new(),
        }
    }
    
    fn change(&mut self, index: i32, number: i32) {
        if let Some(&old_number) = self.index_to_number.get(&index) {
            *self.number_to_index.get_mut(&old_number).unwrap() = -1; // 标记旧索引为-1

        }
        
        self.index_to_number.insert(index, number);
        self.number_to_index.insert(number, index);
        self.numbers.insert(number);
    }
    
    fn find(&self, number: i32) -> i32 {
        if let Some(&index) = self.number_to_index.get(&number) {
            index

        } else {
            -1

        }
    }
}

fn main() {
    let mut obj = NumberContainers::new();
    obj.change(2, 10);
    obj.change(1, 10);
    obj.change(3, 10);
    obj.change(5, 10);
    let ret_2: i32 = obj.find(10);
    obj.change(1, 20);
    let ret_3: i32 = obj.find(10);
}
