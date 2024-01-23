


impl Solution {
    pub fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let mut edges: std::collections::HashMap<i32, Vec<usize>> = std::collections::HashMap::new();
        
        let mut now = -1;
        for (i, &p) in parents.iter().enumerate() {
            edges.entry(p).or_insert(vec![]).push(i);
            if nums[i] == 1 {
                now = i as i32;
            }
        }
        
        let mut ret = vec![1; nums.len()];
        
        if now < 0 {
            return ret;
        }
        
        let mut vis = vec![false; (nums.iter().max().unwrap_or(&0) + 2) as usize];
        
        fn dfs(now: i32, edges: &std::collections::HashMap<i32, Vec<usize>>, nums: &Vec<i32>, ret: &mut Vec<i32>, vis: &mut Vec<bool>) {
            if !vis[nums[now as usize] as usize] {
                vis[nums[now as usize] as usize] = true;
                if let Some(children) = edges.get(&now) {
                    for &nxt in children {
                        dfs(nxt as i32, edges, nums, ret, vis);
                    }
                }
            }
        }
        
        let mut val = 1;
        while now != -1 {
            dfs(now, &edges, &nums, &mut ret, &mut vis);
            while vis[val as usize] {
                val += 1;
            }
            ret[now as usize] = val;
            now = parents[now as usize];
        }
        
        ret

    }
}
