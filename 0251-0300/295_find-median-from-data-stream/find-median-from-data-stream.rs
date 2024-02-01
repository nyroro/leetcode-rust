
use std::collections::BinaryHeap;

struct MedianFinder {
    max_heap: BinaryHeap<i32>,
    min_heap: BinaryHeap<i32>,
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
        }
    }
    
    fn add_num(&mut self, num: i32) {
        if self.max_heap.is_empty() || num <= *self.max_heap.peek().unwrap() {
            self.max_heap.push(num);
        } else {
            self.min_heap.push(-num);
        }
        
        if self.max_heap.len() > self.min_heap.len() + 1 {
            self.min_heap.push(-self.max_heap.pop().unwrap());
        } else if self.min_heap.len() > self.max_heap.len() {
            self.max_heap.push(-self.min_heap.pop().unwrap());
        }
    }
    
    fn find_median(&self) -> f64 {
        if self.max_heap.len() == self.min_heap.len() {
            (*self.max_heap.peek().unwrap() as f64 - *self.min_heap.peek().unwrap() as f64) / 2.0

        } else {
            *self.max_heap.peek().unwrap() as f64

        }
    }
}

fn main() {
    let mut obj = MedianFinder::new();
    obj.add_num(1);
    obj.add_num(2);
    let ret_1: f64 = obj.find_median();
    obj.add_num(3);
    let ret_2: f64 = obj.find_median();
    println!("Median 1: {}", ret_1);
    println!("Median 2: {}", ret_2);
}
