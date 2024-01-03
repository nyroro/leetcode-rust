
impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        
        let mut count_map: HashMap<Vec<i32>, i32> = HashMap::new();
        let mut result = 0;
        
        for domino in dominoes {
            let mut sorted_domino = domino.clone();
            sorted_domino.sort();
            let count = count_map.entry(sorted_domino).or_insert(0);
            *count += 1;
        }
        
        for count in count_map.values() {
            result += count * (count - 1) / 2;
        }
        
        result

    }
}
