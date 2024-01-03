
impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut groups: std::collections::HashMap<i32, Vec<i32>> = std::collections::HashMap::new();
        
        for (i, &size) in group_sizes.iter().enumerate() {
            let group = groups.entry(size).or_insert(Vec::new());
            group.push(i as i32);
            
            if group.len() == size as usize {
                result.push(group.clone());
                *group = Vec::new();
            }
        }
        
        result

    }
}
