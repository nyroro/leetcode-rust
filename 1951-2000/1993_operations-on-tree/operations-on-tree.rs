
use std::collections::HashMap;

struct TreeNode {
    id: i32,
    locked: bool,
    locked_by: Option<i32>,
    children: Vec<i32>,
}

struct LockingTree {
    nodes: HashMap<i32, TreeNode>,
    parent: Vec<i32>,
}

impl LockingTree {
    fn new(parent: Vec<i32>) -> Self {
        let mut nodes = HashMap::new();
        for (i, &p) in parent.iter().enumerate() {
            nodes.insert(i as i32, TreeNode {
                id: i as i32,
                locked: false,
                locked_by: None,
                children: vec![],
            });
        }
        for (i, &p) in parent.iter().enumerate() {
            if p != -1 {
                nodes.get_mut(&p).unwrap().children.push(i as i32);
            }
        }
        LockingTree {
            nodes,
            parent,
        }
    }
    
    fn lock(&mut self, num: i32, user: i32) -> bool {
        let node = self.nodes.get_mut(&num).unwrap();
        if !node.locked {
            node.locked = true;
            node.locked_by = Some(user);
            true

        } else {
            false

        }
    }
    
    fn unlock(&mut self, num: i32, user: i32) -> bool {
        let node = self.nodes.get_mut(&num).unwrap();
        if node.locked && node.locked_by == Some(user) {
            node.locked = false;
            node.locked_by = None;
            true

        } else {
            false

        }
    }
    
    fn upgrade(&mut self, num: i32, user: i32) -> bool {
        let node = self.nodes.get_mut(&num).unwrap();
        if !node.locked && self.has_locked_descendant(num) && !self.has_locked_ancestor(num) {
            self.unlock_descendants(num);
            node.locked = true;
            node.locked_by = Some(user);
            true

        } else {
            false

        }
    }

    fn has_locked_descendant(&self, num: i32) -> bool {
        let mut stack = vec![num];
        while let Some(curr) = stack.pop() {
            let node = self.nodes.get(&curr).unwrap();
            if node.locked {
                return true;
            }
            for &child in &node.children {
                stack.push(child);
            }
        }
        false

    }

    fn has_locked_ancestor(&self, num: i32) -> bool {
        let mut curr = num;
        while curr != -1 {
            let node = self.nodes.get(&curr).unwrap();
            if node.locked {
                return true;
            }
            curr = self.parent[curr as usize];
        }
        false

    }

    fn unlock_descendants(&mut self, num: i32) {
        let mut stack = vec![num];
        while let Some(curr) = stack.pop() {
            let node = self.nodes.get_mut(&curr).unwrap();
            if node.locked {
                node.locked = false;
                node.locked_by = None;
            }
            for &child in &node.children {
                stack.push(child);
            }
        }
    }
}

fn main() {
    let mut locking_tree = LockingTree::new(vec![-1, 0, 0, 1, 1, 2, 2]);
    println!("{}", locking_tree.lock(2, 2));    // true

    println!("{}", locking_tree.unlock(2, 3));  // false

    println!("{}", locking_tree.unlock(2, 2));  // true

    println!("{}", locking_tree.lock(4, 5));    // true

    println!("{}", locking_tree.upgrade(0, 1)); // true

    println!("{}", locking_tree.lock(0, 1));    // false

}
