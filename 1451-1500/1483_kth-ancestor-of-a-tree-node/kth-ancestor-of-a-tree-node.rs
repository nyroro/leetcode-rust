
struct TreeAncestor {
    ancestors: Vec<Vec<i32>>,
}

impl TreeAncestor {
    fn new(n: i32, parent: Vec<i32>) -> Self {
        let mut ancestors = vec![vec![-1; 16]; n as usize]; // 16是一个足够大的值，保证能够覆盖n的范围

        for i in 0..n {
            ancestors[i as usize][0] = parent[i as usize];
        }
        for j in 1..16 {
            for i in 0..n {
                if ancestors[i as usize][j - 1] != -1 {
                    ancestors[i as usize][j] = ancestors[ancestors[i as usize][j - 1] as usize][j - 1];
                }
            }
        }
        TreeAncestor { ancestors }
    }
    
    fn get_kth_ancestor(&self, node: i32, k: i32) -> i32 {
        let mut node = node;
        let mut k = k;
        for i in 0..16 {
            if (k >> i) & 1 != 0 {
                if node == -1 {
                    return -1;
                }
                node = self.ancestors[node as usize][i];
            }
        }
        node

    }
}

fn main() {
    let tree_ancestor = TreeAncestor::new(7, vec![-1, 0, 0, 1, 1, 2, 2]);
    println!("{}", tree_ancestor.get_kth_ancestor(3, 1)); // returns 1 which is the parent of 3

    println!("{}", tree_ancestor.get_kth_ancestor(5, 2)); // returns 0 which is the grandparent of 5

    println!("{}", tree_ancestor.get_kth_ancestor(6, 3)); // returns -1 because there is no such ancestor

}
