
impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut reachable = vec![false; n as usize];
        
        for edge in edges {
            reachable[edge[1] as usize] = true;
        }
        
        let mut result = Vec::new();
        
        for i in 0..n {
            if !reachable[i as usize] {
                result.push(i);
            }
        }
        
        result

    }
}
