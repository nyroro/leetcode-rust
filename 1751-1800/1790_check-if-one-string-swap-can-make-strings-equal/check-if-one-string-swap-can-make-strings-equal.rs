
impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        if s1 == s2 {
            return true;
        }
        
        let mut diff_count = 0;
        let mut diff_indices: Vec<usize> = Vec::new();
        
        for (i, (c1, c2)) in s1.chars().zip(s2.chars()).enumerate() {
            if c1 != c2 {
                diff_count += 1;
                diff_indices.push(i);
            }
            
            if diff_count > 2 {
                return false;
            }
        }
        
        if diff_count == 0 {
            return true;
        }
        
        if diff_count == 2 {
            let index1 = diff_indices[0];
            let index2 = diff_indices[1];
            
            let chars1: Vec<char> = s1.chars().collect();
            let chars2: Vec<char> = s2.chars().collect();
            
            chars1[index1] == chars2[index2] && chars1[index2] == chars2[index1]
        } else {
            false

        }
    }
}
