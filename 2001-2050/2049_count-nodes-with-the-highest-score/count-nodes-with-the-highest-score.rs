
struct TreeNode {
    index: usize,
    parent: i32,
    children: Vec<usize>,
}

impl Solution {
    pub fn count_highest_score_nodes(parents: Vec<i32>) -> i32 {
        let n = parents.len();
        let mut nodes = vec![TreeNode { index: 0, parent: -1, children: vec![] }; n];
        
        // 构建树结构

        for i in 1..n {
            let parent = parents[i as usize] as usize;
            nodes[i].index = i;
            nodes[i].parent = parent as i32;
            nodes[parent].children.push(i);
        }
        
        let mut scores = vec![1; n];
        let mut subtree_sizes = vec![1; n];
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        
        // 找到叶子节点

        for i in 0..n {
            if nodes[i].children.is_empty() {
                queue.push_back(i);
                visited.insert(i);
            }
        }
        
        // 计算得分和子节点数

        while let Some(current) = queue.pop_front() {
            let parent = nodes[current].parent as usize;
            if parent != usize::MAX {
                scores[current] *= n - subtree_sizes[current];
                scores[parent] *= subtree_sizes[current];
                subtree_sizes[parent] += subtree_sizes[current];
                nodes[parent].children.retain(|&x| x != current);
                if nodes[parent].children.is_empty() && !visited.contains(&parent) {
                    visited.insert(parent);
                    queue.push_back(parent);
                }
            }
        }
        
        // 找到最高得分

        let max_score = *scores.iter().max().unwrap();
        
        // 统计得分等于最高得分的节点数量

        let count = scores.iter().filter(|&&x| x == max_score).count() as i32;
        
        count

    }
}
