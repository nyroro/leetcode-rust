
use std::collections::{HashMap, HashSet};

struct ThroneInheritance {
    king: String,
    children: HashMap<String, Vec<String>>,
    dead: HashSet<String>,
}

impl ThroneInheritance {
    fn new(king_name: String) -> Self {
        ThroneInheritance {
            king: king_name,
            children: HashMap::new(),
            dead: HashSet::new(),
        }
    }

    fn birth(&mut self, parent_name: String, child_name: String) {
        self.children.entry(parent_name).or_insert(vec![]).push(child_name);
    }

    fn death(&mut self, name: String) {
        self.dead.insert(name);
    }

    fn get_inheritance_order(&self) -> Vec<String> {
        let mut order = vec![];
        self.dfs(&self.king, &mut order);
        order

    }

    fn dfs(&self, person: &String, order: &mut Vec<String>) {
        if !self.dead.contains(person) {
            order.push(person.clone());
        }
        if let Some(children) = self.children.get(person) {
            for child in children {
                self.dfs(child, order);
            }
        }
    }
}
