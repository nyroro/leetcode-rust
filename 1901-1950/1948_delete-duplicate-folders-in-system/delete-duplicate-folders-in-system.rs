
use std::collections::{HashMap, HashSet};

struct Trie {
    children: HashMap<String, Trie>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, path: &[String]) {
        let mut node = self;
        for p in path {
            node = node.children.entry(p.clone()).or_insert(Trie::new());
        }
    }

    fn deduplicate(&mut self, seen: &mut HashMap<String, *mut Trie>, to_delete: &mut HashSet<*mut Trie>) -> String {
        let mut path = String::new();
        for (s, n) in &mut self.children {
            path.push_str(&s);
            path.push_str(&n.deduplicate(seen, to_delete));
        }
        if path.is_empty() {
            return String::new();
        }
        if seen.contains_key(&path) {
            to_delete.insert(seen[&path]);
            to_delete.insert(self as *mut Trie);
        } else {
            seen.insert(path.clone(), self as *mut Trie);
        }
        format!("({})", path)
    }

    fn get_remaining_paths(&self, to_delete: &HashSet<*mut Trie>, paths: &mut Vec<Vec<String>>, path: &mut Vec<String>) {
        for (s, n) in &self.children {
            if to_delete.contains(&(n as *const Trie as *mut Trie)) {
                continue;
            }
            path.push(s.clone());
            n.get_remaining_paths(to_delete, paths, path);
            path.pop();
        }
        if !path.is_empty() {
            paths.push(path.clone());
        }
    }
}

impl Solution {
    pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut root = Trie::new();
        for path in paths {
            root.insert(&path);
        }
        let mut seen: HashMap<String, *mut Trie> = HashMap::new();
        let mut to_delete: HashSet<*mut Trie> = HashSet::new();
        let _ = root.deduplicate(&mut seen, &mut to_delete);
        let mut remaining_paths: Vec<Vec<String>> = Vec::new();
        let mut path: Vec<String> = Vec::new();
        root.get_remaining_paths(&to_delete, &mut remaining_paths, &mut path);
        remaining_paths

    }
}
