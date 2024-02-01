
use std::collections::{HashSet, VecDeque};



impl Solution {
    pub fn k_similarity(s1: String, s2: String) -> i32 {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((s1, 0));
        visited.insert(s1.clone());

        while let Some((current, swaps)) = queue.pop_front() {
            if current == s2 {
                return swaps;
            }

            let current_bytes = current.as_bytes();
            let s2_bytes = s2.as_bytes();
            let mut i = 0;
            while current_bytes[i] == s2_bytes[i] {
                i += 1;
            }

            for j in (i + 1)..current.len() {
                if current_bytes[j] == s2_bytes[i] && current_bytes[j] != s2_bytes[j] {
                    let mut next = current_bytes.to_vec();
                    next.swap(i, j);
                    let next_str = String::from_utf8(next).unwrap();
                    if !visited.contains(&next_str) {
                        queue.push_back((next_str, swaps + 1));
                        visited.insert(next_str);
                    }
                }
            }
        }

        0

    }
}

fn main() {
    let s1 = String::from("ab");
    let s2 = String::from("ba");
    println!("{}", Solution::k_similarity(s1, s2));  // Output: 1


    let s1 = String::from("abc");
    let s2 = String::from("bca");
    println!("{}", Solution::k_similarity(s1, s2));  // Output: 2

}
