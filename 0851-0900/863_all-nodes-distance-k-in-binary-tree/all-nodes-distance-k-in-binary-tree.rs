
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn distance_k(root: Option<Rc<RefCell<TreeNode>>>, target: Option<Rc<RefCell<TreeNode>>>, k: i32) -> Vec<i32> {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut result: Vec<i32> = Vec::new();
        
        // 构建图

        fn build_graph(node: Option<Rc<RefCell<TreeNode>>>, parent: Option<i32>, graph: &mut HashMap<i32, Vec<i32>>) {
            if let Some(n) = node {
                let val = n.borrow().val;
                if let Some(p) = parent {
                    graph.entry(val).or_insert(vec![]).push(p);
                    graph.entry(p).or_insert(vec![]).push(val);
                }
                build_graph(n.borrow().left.clone(), Some(val), graph);
                build_graph(n.borrow().right.clone(), Some(val), graph);
            }
        }
        
        // 从目标节点开始DFS

        fn dfs(node: i32, distance: i32, graph: &HashMap<i32, Vec<i32>>, visited: &mut Vec<bool>, result: &mut Vec<i32>) {
            visited[node as usize] = true;
            if distance == 0 {
                result.push(node);
                return;
            }
            for &neighbor in graph[&node].iter() {
                if !visited[neighbor as usize] {
                    dfs(neighbor, distance - 1, graph, visited, result);
                }
            }
        }
        
        build_graph(root, None, &mut graph);
        
        let target_val = target.unwrap().borrow().val;
        
        let mut visited = vec![false; 501];
        dfs(target_val, k, &graph, &mut visited, &mut result);
        
        result

    }
}
