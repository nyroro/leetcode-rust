
use std::collections::HashMap;

struct DisjointSet {
    parent: Vec<usize>,
    smallest_char: Vec<char>,
}

impl DisjointSet {
    fn new() -> DisjointSet {
        let mut parent = Vec::new();
        let mut smallest_char = Vec::new();
        for i in 0..26 {
            parent.push(i);
            smallest_char.push((b'a' + i as u8) as char);
        }
        DisjointSet { parent, smallest_char }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while x != self.parent[x] {
            x = self.parent[x];
        }
        x

    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            if self.smallest_char[root_x] < self.smallest_char[root_y] {
                self.parent[root_y] = root_x;
            } else {
                self.parent[root_x] = root_y;
            }
        }
    }
}



impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let mut disjoint_set = DisjointSet::new();
        let mut char_map: HashMap<char, usize> = HashMap::new();

        for (c1, c2) in s1.chars().zip(s2.chars()) {
            let idx1 = (c1 as u8 - b'a') as usize;
            let idx2 = (c2 as u8 - b'a') as usize;
            disjoint_set.union(idx1, idx2);
        }

        let mut ans = String::new();
        for c in base_str.chars() {
            let idx = (c as u8 - b'a') as usize;
            let root = disjoint_set.find(idx);
            ans.push(disjoint_set.smallest_char[root]);
        }

        ans

    }
}
