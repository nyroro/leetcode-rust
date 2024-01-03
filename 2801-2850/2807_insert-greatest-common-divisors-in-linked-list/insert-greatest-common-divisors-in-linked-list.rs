
// 辅助函数：计算两个数的最大公约数

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs()
}

// 实现插入最大公约数的函数

impl Solution {
    pub fn insert_greatest_common_divisors(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = &mut head;
        
        while let Some(node) = current {
            if let Some(mut next_node) = node.next.take() {
                let new_val = gcd(node.val, next_node.val);
                let new_node = Some(Box::new(ListNode { val: new_val, next: Some(next_node) }));
                node.next = new_node;
                current = &mut node.next.as_mut().unwrap().next;
            } else {
                break;
            }
        }
        
        head

    }
}
