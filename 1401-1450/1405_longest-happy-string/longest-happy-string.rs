
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct CharCount {
    ch: char,
    count: i32,
}

impl PartialEq for CharCount {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count

    }
}

impl Eq for CharCount {}

impl PartialOrd for CharCount {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.count.cmp(&other.count))
    }
}

impl Ord for CharCount {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.count.cmp(&other.count)
    }
}

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut heap = BinaryHeap::new();
        if a > 0 {
            heap.push(Reverse(CharCount { ch: 'a', count: a }));
        }
        if b > 0 {
            heap.push(Reverse(CharCount { ch: 'b', count: b }));
        }
        if c > 0 {
            heap.push(Reverse(CharCount { ch: 'c', count: c }));
        }

        let mut result = String::new();
        while let Some(Reverse(mut max_char)) = heap.pop() {
            if result.len() >= 2 && result.chars().last().unwrap() == max_char.ch && result.chars().nth(result.len() - 2).unwrap() == max_char.ch {
                if let Some(Reverse(mut second_char)) = heap.pop() {
                    result.push(second_char.ch);
                    second_char.count -= 1;
                    if second_char.count > 0 {
                        heap.push(Reverse(second_char));
                    }
                    heap.push(Reverse(max_char));
                } else {
                    break;
                }
            } else {
                result.push(max_char.ch);
                max_char.count -= 1;
                if max_char.count > 0 {
                    heap.push(Reverse(max_char));
                }
            }
        }
        result

    }
}
