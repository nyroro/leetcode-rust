
impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut greedy = g;
        let mut cookies = s;
        greedy.sort();
        cookies.sort();
        
        let mut count = 0;
        let mut i = 0;
        let mut j = 0;
        
        while i < greedy.len() && j < cookies.len() {
            if cookies[j] >= greedy[i] {
                count += 1;
                i += 1;
            }
            j += 1;
        }
        
        count

    }
}
