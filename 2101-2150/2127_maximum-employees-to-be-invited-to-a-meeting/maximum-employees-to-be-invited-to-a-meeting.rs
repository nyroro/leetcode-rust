


impl Solution {
    pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
        let n = favorite.len();
        let mut indegree = vec![0; n];
        
        for &f in &favorite {
            indegree[f as usize] += 1;
        }
        
        let mut length = vec![0; n];
        let mut visited = vec![false; n];
        let mut q = Vec::new();
        let mut qi = 0;
        for (i, &t) in indegree.iter().enumerate() {
            if t == 0 {
                q.push(i as i32);
            }
        }
        while qi < q.len() {
            let now = q[qi];
            qi += 1;
            visited[now as usize] = true;
            let nxt = favorite[now as usize];
            length[nxt as usize] = length[nxt as usize].max(length[now as usize] + 1);
            indegree[nxt as usize] -= 1;
            if indegree[nxt as usize] == 0 {
                q.push(nxt);
            }
        }
        let mut acyclic = 0;
        let mut cyclic = 0;
        
        for i in 0..n {
            if !visited[i] {
                let mut now_len = 0;
                let mut now = i;
                while !visited[now] {
                    now_len += 1;
                    visited[now] = true;
                    now = favorite[now as usize];
                }
                if now_len == 2 {
                    acyclic += now_len + length[i] + length[favorite[i as usize]];
                } else {
                    cyclic = cyclic.max(now_len);
                }
            }
        }
        acyclic.max(cyclic)
    }
}
