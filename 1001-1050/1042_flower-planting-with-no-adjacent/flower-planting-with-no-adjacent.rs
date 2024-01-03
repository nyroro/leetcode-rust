
impl Solution {
    pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
        // Step 1: Create an array to represent the types of gardens

        let mut flowers = vec![0; n as usize];
        
        // Step 2: Create a hash map to store adjacent gardens

        let mut adj_map: std::collections::HashMap<i32, Vec<i32>> = std::collections::HashMap::new();
        
        // Step 3: Store adjacent garden information in the hash map

        for path in &paths {
            adj_map.entry(path[0]).or_insert(vec![]).push(path[1]);
            adj_map.entry(path[1]).or_insert(vec![]).push(path[0]);
        }
        
        // Step 4: Choose a flower type for each garden

        for garden in 1..=n {
            let mut types = vec![false; 5];
            if let Some(adjacent) = adj_map.get(&garden) {
                for &adj in adjacent {
                    types[flowers[adj as usize - 1] as usize] = true;
                }
            }
            for t in 1..=4 {
                if !types[t] {
                    flowers[garden as usize - 1] = t;
                    break;
                }
            }
        }
        
        // Step 5: Return the array of garden types as the result

        flowers.iter().map(|&x| x as i32).collect()
    }
}
