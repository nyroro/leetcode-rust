
impl Solution {
    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        let mut ind: Vec<i32> = vec![0; n as usize];
        let mut outd: Vec<i32> = vec![0; n as usize];
        let mut e = 0;

        for i in 0..n {
            if left_child[i as usize] >= 0 {
                e += 1;
                ind[left_child[i as usize] as usize] += 1;
            }
            if right_child[i as usize] >= 0 {
                e += 1;
                ind[right_child[i as usize] as usize] += 1;
            }
        }

        if e != n - 1 {
            return false;
        }

        let mut root = -1;
        for i in 0..n {
            if ind[i as usize] > 1 {
                return false;
            }
            if ind[i as usize] == 0 {
                if root < 0 {
                    root = i;
                } else {
                    return false;
                }
            }
        }

        let mut vis: Vec<bool> = vec![false; n as usize];

        fn dfs(root: i32, left_child: &Vec<i32>, right_child: &Vec<i32>, vis: &mut Vec<bool>) {
            vis[root as usize] = true;
            if left_child[root as usize] >= 0 && !vis[left_child[root as usize] as usize] {
                dfs(left_child[root as usize], left_child, right_child, vis);
            }
            if right_child[root as usize] >= 0 && !vis[right_child[root as usize] as usize] {
                dfs(right_child[root as usize], left_child, right_child, vis);
            }
        }

        dfs(root, &left_child, &right_child, &mut vis);

        vis.iter().all(|&x| x)
    }
}
