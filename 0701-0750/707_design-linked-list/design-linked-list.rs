
// Define the Node struct

struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

// Define the MyLinkedList struct

pub struct MyLinkedList {
    head: Option<Box<Node>>,
    size: i32,
}

// Implementation of MyLinkedList methods

impl MyLinkedList {
    // Initialize an empty linked list

    pub fn new() -> Self {
        MyLinkedList { head: None, size: 0 }
    }

    // Retrieve the value of the node at the specified index

    pub fn get(&self, index: i32) -> i32 {
        if index < 0 || index >= self.size {
            return -1;
        }
        let mut current = &self.head;
        let mut i = 0;
        while let Some(node) = current {
            if i == index {
                return node.val;
            }
            current = &node.next;
            i += 1;
        }
        -1

    }

    // Add a new node with the given value at the head of the linked list

    pub fn add_at_head(&mut self, val: i32) {
        let new_node = Node {
            val,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
        self.size += 1;
    }

    // Append a new node with the given value at the end of the linked list

    pub fn add_at_tail(&mut self, val: i32) {
        if self.size == 0 {
            self.add_at_head(val);
            return;
        }
        let mut current = &mut self.head;
        while let Some(node) = current {
            if node.next.is_none() {
                let new_node = Node { val, next: None };
                node.next = Some(Box::new(new_node));
                self.size += 1;
                return;
            }
            current = &mut node.next;
        }
    }

    // Add a new node with the given value at the specified index

    pub fn add_at_index(&mut self, index: i32, val: i32) {
        if index > self.size {
            return;
        }
        if index <= 0 {
            self.add_at_head(val);
            return;
        }
        let mut current = &mut self.head;
        let mut i = 0;
        while let Some(node) = current {
            if i == index - 1 {
                let new_node = Node {
                    val,
                    next: node.next.take(),
                };
                node.next = Some(Box::new(new_node));
                self.size += 1;
                return;
            }
            current = &mut node.next;
            i += 1;
        }
    }

    // Delete the node at the specified index

    pub fn delete_at_index(&mut self, index: i32) {
        if index < 0 || index >= self.size {
            return;
        }
        if index == 0 {
            self.head = self.head.as_mut().unwrap().next.take();
            self.size -= 1;
            return;
        }
        let mut current = &mut self.head;
        let mut i = 0;
        while let Some(node) = current {
            if i == index - 1 {
                node.next = node.next.as_mut().unwrap().next.take();
                self.size -= 1;
                return;
            }
            current = &mut node.next;
            i += 1;
        }
    }
}
