
impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut parent = vec![0; n + 1];
        
        for i in 1..=n {
            parent[i] = i;
        }
        
        for edge in edges {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            
            if Solution::find(&parent, a) == Solution::find(&parent, b) {
                return edge;
            }
            
            Solution::union(&mut parent, a, b);
        }
        
        vec![]
    }
    
    fn find(parent: &[usize], x: usize) -> usize {
        if parent[x] != x {
            Solution::find(parent, parent[x])
        } else {
            x

        }
    }
    
    fn union(parent: &mut [usize], x: usize, y: usize) {
        let root_x = Solution::find(parent, x);
        let root_y = Solution::find(parent, y);
        
        if root_x != root_y {
            parent[root_x] = root_y;
        }
    }
}
