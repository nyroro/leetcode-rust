
use std::collections::HashMap;

struct MajorityChecker {
    arr: Vec<i32>,
    cache: HashMap<(i32, i32), (i32, i32)>,
}

impl MajorityChecker {
    fn new(arr: Vec<i32>) -> Self {
        MajorityChecker {
            arr,
            cache: HashMap::new(),
        }
    }

    fn query(&mut self, left: i32, right: i32, threshold: i32) -> i32 {
        if let Some(&(count, element)) = self.cache.get(&(left, right)) {
            if count >= threshold {
                return element;
            } else {
                return -1;
            }
        }

        let (mut element, mut vote) = (-1, 0);
        for i in left..=right {
            if vote == 0 {
                element = self.arr[i as usize];
            }
            if element == self.arr[i as usize] {
                vote += 1;
            } else {
                vote -= 1;
            }
        }

        let count = self.arr.iter().skip(left as usize).take((right - left + 1) as usize).filter(|&x| *x == element).count() as i32;
        self.cache.insert((left, right), (count, element));

        if count >= threshold {
            element

        } else {
            -1

        }
    }
}
