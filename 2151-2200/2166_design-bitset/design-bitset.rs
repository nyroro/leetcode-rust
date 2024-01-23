
use std::collections::HashSet;

struct Bitset {
    ones: HashSet<i32>,
    zeros: HashSet<i32>,
    size: i32,
}

impl Bitset {
    fn new(size: i32) -> Self {
        let mut zeros = HashSet::new();
        for i in 0..size {
            zeros.insert(i);
        }
        Bitset {
            ones: HashSet::new(),
            zeros,
            size,
        }
    }

    fn fix(&mut self, idx: i32) {
        self.ones.insert(idx);
        self.zeros.remove(&idx);
    }

    fn unfix(&mut self, idx: i32) {
        self.ones.remove(&idx);
        self.zeros.insert(idx);
    }

    fn flip(&mut self) {
        std::mem::swap(&mut self.ones, &mut self.zeros);
    }

    fn all(&self) -> bool {
        self.zeros.is_empty()
    }

    fn one(&self) -> bool {
        !self.ones.is_empty()
    }

    fn count(&self) -> i32 {
        self.ones.len() as i32

    }

    fn to_string(&self) -> String {
        let mut result = String::new();
        for i in 0..self.size {
            if self.ones.contains(&i) {
                result.push('1');
            } else {
                result.push('0');
            }
        }
        result

    }
}
