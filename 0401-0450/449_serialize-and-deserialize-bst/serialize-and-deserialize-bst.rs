
use std::rc::Rc;
use std::cell::RefCell;

struct Codec {}

impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut result = String::new();
        self.serialize_helper(&root, &mut result);
        result.pop(); // 移除末尾的逗号

        result

    }

    fn serialize_helper(&self, root: &Option<Rc<RefCell<TreeNode>>>, result: &mut String) {
        match root {
            Some(node) => {
                result.push_str(&node.borrow().val.to_string());
                result.push(',');
                self.serialize_helper(&node.borrow().left, result);
                self.serialize_helper(&node.borrow().right, result);
            }
            None => {
                result.push_str("#,");
            }
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let nodes: Vec<&str> = data.split(',').collect();
        let mut iter = nodes.iter();
        self.deserialize_helper(&mut iter)
    }

    fn deserialize_helper(&self, iter: &mut std::slice::Iter<&str>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(val) = iter.next() {
            if val == &"#" {
                return None;
            } else {
                let val = val.parse::<i32>().unwrap();
                let mut node = TreeNode::new(val);
                node.left = self.deserialize_helper(iter);
                node.right = self.deserialize_helper(iter);
                return Some(Rc::new(RefCell::new(node)));
            }
        }
        None

    }
}
