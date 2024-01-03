
impl Solution {
    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        let n = s.len();
        let mut parent = vec![0; n];
        let mut rank = vec![0; n];
        let mut chars = s.chars().collect::<Vec<char>>();

        // 初始化并查集

        for i in 0..n {
            parent[i] = i;
        }

        // 合并连通分量

        for pair in pairs {
            let x = pair[0] as usize;
            let y = pair[1] as usize;
            Solution::union(&mut parent, &mut rank, x, y);
        }

        // 构建连通分量的字符集合

        let mut groups = vec![vec![]; n];
        for i in 0..n {
            let root = Solution::find(&mut parent, i);
            groups[root].push(chars[i]);
        }

        // 对每个连通分量的字符进行排序

        for group in &mut groups {
            group.sort_unstable_by(|a, b| b.cmp(a));
        }

        // 替换原字符串中的字符

        for i in 0..n {
            let root = Solution::find(&mut parent, i);
            chars[i] = groups[root].pop().unwrap();
        }

        chars.into_iter().collect()
    }

    // 查找根节点

    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x {
            parent[x] = Solution::find(parent, parent[x]);
        }
        parent[x]
    }

    // 合并连通分量

    fn union(parent: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) {
        let root_x = Solution::find(parent, x);
        let root_y = Solution::find(parent, y);
        if root_x != root_y {
            if rank[root_x] < rank[root_y] {
                parent[root_x] = root_y;
            } else if rank[root_x] > rank[root_y] {
                parent[root_y] = root_x;
            } else {
                parent[root_y] = root_x;
                rank[root_x] += 1;
            }
        }
    }
}
