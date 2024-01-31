
use std::collections::{HashMap, HashSet};

fn is_tree(graph: &Vec<Vec<usize>>) -> bool {
    let n = graph.len();
    let mut visited = HashSet::new();

    fn dfs(u: usize, parent: Option<usize>, visited: &mut HashSet<usize>, graph: &Vec<Vec<usize>>) -> bool {
        visited.insert(u);

        let mut has_cycle = false;

        for &v in &graph[u] {
            if Some(v as usize) == parent {
                continue;
            }
            if visited.contains(&v) {
                return true;
            }
            has_cycle |= dfs(v, Some(u), visited, graph);
        }

        has_cycle

    }

    !dfs(0, None, &mut visited, &graph) && visited.len() == n

}

fn diameter(tree_graph: &Vec<Vec<usize>>) -> usize {
    let mut x = 0;
    let mut dis = 0;

    fn dfs(u: usize, p: Option<usize>, d: usize, x: &mut usize, dis: &mut usize, tree_graph: &Vec<Vec<usize>>) {
        if d > *dis {
            *dis = d;
            *x = u;
        }
        for &v in &tree_graph[u] {
            if Some(v as usize) == p {
                continue;
            }
            dfs(v, Some(u), d + 1, x, dis, tree_graph);
        }
    }

    dfs(0, None, 0, &mut x, &mut dis, &tree_graph);
    let mut dis = 0;
    dfs(x, None, 0, &mut x, &mut dis, &tree_graph);

    dis

}

impl Solution {
    pub fn count_subgraphs_for_each_diameter(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut ans = vec![0; n - 1];

        for i in 1..(1 << (n - 1)) {
            let mut use_edges = vec![];
            let mut nodes = HashSet::new();

            for j in 0..(n - 1) {
                if (i >> j) & 1 == 1 {
                    use_edges.push(edges[j].clone());
                    nodes.insert(edges[j][0] as usize);
                    nodes.insert(edges[j][1] as usize);
                }
            }

            let node_count = nodes.len();
            if node_count <= 1 {
                continue;
            }

            let mut node_to_node_id = HashMap::new();
            for (node_id, &node) in nodes.iter().enumerate() {
                node_to_node_id.insert(node, node_id);
            }

            let mut graph = vec![vec![]; node_count];
            for edge in &use_edges {
                let a = edge[0] as usize;
                let b = edge[1] as usize;
                let a_id = *node_to_node_id.get(&a).unwrap();
                let b_id = *node_to_node_id.get(&b).unwrap();
                graph[a_id].push(b_id);
                graph[b_id].push(a_id);
            }

            if is_tree(&graph) {
                let d = diameter(&graph);
                ans[d - 1] += 1;
            }
        }

        ans.iter().map(|&x| x as i32).collect()
    }
}
