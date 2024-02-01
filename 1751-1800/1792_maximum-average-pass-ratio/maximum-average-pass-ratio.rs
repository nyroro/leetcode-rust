
use std::collections::BinaryHeap;
use std::cmp::Ordering;

struct Class {
    pass: i32,
    total: i32,
}

impl Class {
    fn pass_ratio_increase(&self, extra_students: i32) -> f64 {
        let pass = self.pass as f64 + extra_students as f64;
        let total = self.total as f64 + extra_students as f64;
        (pass / total) - (self.pass as f64 / self.total as f64)
    }
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        (self.pass_ratio_increase(1)).partial_cmp(&other.pass_ratio_increase(1)) == Some(Ordering::Equal)
    }
}

impl Eq for Class {}

impl PartialOrd for Class {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.pass_ratio_increase(1)).partial_cmp(&other.pass_ratio_increase(1))
    }
}

impl Ord for Class {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.pass_ratio_increase(1)).partial_cmp(&other.pass_ratio_increase(1)).unwrap()
    }
}



impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let mut max_heap = BinaryHeap::new();
        let mut total_pass_ratio = 0.0;

        for c in &classes {
            let class = Class { pass: c[0], total: c[1] };
            total_pass_ratio += class.pass as f64 / class.total as f64;
            max_heap.push(class);
        }

        for _ in 0..extra_students {
            let max_class = max_heap.pop().unwrap();
            total_pass_ratio -= max_class.pass as f64 / max_class.total as f64;
            max_class.pass += 1;
            max_class.total += 1;
            total_pass_ratio += max_class.pass as f64 / max_class.total as f64;
            max_heap.push(max_class);
        }

        total_pass_ratio / classes.len() as f64

    }
}
