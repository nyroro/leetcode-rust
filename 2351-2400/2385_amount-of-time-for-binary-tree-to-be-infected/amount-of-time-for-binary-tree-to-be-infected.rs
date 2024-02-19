
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;



impl Solution {
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut vis: HashMap<i32, bool> = HashMap::new();
        let mut res = 0;

        // Define a helper function to build the graph

        fn build_graph(node: &Option<Rc<RefCell<TreeNode>>>, graph: &mut HashMap<i32, Vec<i32>>) {
            if let Some(n) = node {
                let val = n.borrow().val;
                graph.entry(val).or_insert(Vec::new());
                if let Some(left) = &n.borrow().left {
                    let l = left.borrow().val;
                    graph.entry(l).or_insert(Vec::new()).push(val);
                    graph.entry(val).or_insert(Vec::new()).push(l);
                    build_graph(&n.borrow().left, graph);
                }
                if let Some(right) = &n.borrow().right {
                    let r = right.borrow().val;
                    graph.entry(r).or_insert(Vec::new()).push(val);
                    graph.entry(val).or_insert(Vec::new()).push(r);
                    build_graph(&n.borrow().right, graph);
                }
            }
        }

        // Define a helper function for DFS

        fn dfs(node: i32, w: i32, graph: &HashMap<i32, Vec<i32>>, vis: &mut HashMap<i32, bool>, res: &mut i32) {
            vis.insert(node, true);
            *res = (*res).max(w);
            if let Some(neighbors) = graph.get(&node) {
                for &child in neighbors {
                    if !vis.get(&child).unwrap_or(&false).clone() {
                        dfs(child, w + 1, graph, vis, res);
                    }
                }
            }
        }

        // Build the graph

        build_graph(&root, &mut graph);

        // Perform DFS

        dfs(start, 0, &graph, &mut vis, &mut res);

        res

    }
}
