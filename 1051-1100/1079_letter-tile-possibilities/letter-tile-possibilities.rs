
impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut result = std::collections::HashSet::new();
        let tiles: Vec<char> = tiles.chars().collect();
        Solution::backtrack(&tiles, &mut vec![false; tiles.len()], &mut String::new(), &mut result);
        result.len() as i32

    }
    
    fn backtrack(tiles: &Vec<char>, used: &mut Vec<bool>, current: &mut String, result: &mut std::collections::HashSet<String>) {
        if current.len() > 0 {
            result.insert(current.clone());
        }
        
        for i in 0..tiles.len() {
            if used[i] {
                continue;
            }
            used[i] = true;
            current.push(tiles[i]);
            Solution::backtrack(tiles, used, current, result);
            current.pop();
            used[i] = false;
        }
    }
}
