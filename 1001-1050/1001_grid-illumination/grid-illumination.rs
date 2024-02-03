
use std::collections::{HashMap, HashSet};



impl Solution {
    pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        // Create HashMaps to store the count of lamps in each row, column, and diagonal

        let mut rows = HashMap::new();
        let mut cols = HashMap::new();
        let mut pd = HashMap::new();
        let mut sd = HashMap::new();
        
        // Create a HashSet to store the illuminated cells

        let mut lighted = HashSet::new();
        
        // Iterate through the lamps and update the HashMaps and HashSet

        for lamp in &lamps {
            let (a, b) = (lamp[0], lamp[1]);
            if lighted.contains(&(a, b)) {
                continue;
            }
            *rows.entry(a).or_insert(0) += 1;
            *cols.entry(b).or_insert(0) += 1;
            *pd.entry(a - b).or_insert(0) += 1;
            *sd.entry(a + b).or_insert(0) += 1;
            lighted.insert((a, b));
        }
        
        // Define a list of moves to check the adjacent cells

        let moves = vec![(0, 0), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)];
        
        // Create a vector to store the results

        let mut res = Vec::new();
        
        // Iterate through the queries and check if the cell is illuminated

        for query in &queries {
            let (a, b) = (query[0], query[1]);
            if rows.get(&a).cloned().unwrap_or(0) > 0

                || cols.get(&b).cloned().unwrap_or(0) > 0

                || pd.get(&(a - b)).cloned().unwrap_or(0) > 0

                || sd.get(&(a + b)).cloned().unwrap_or(0) > 0

            {
                res.push(1);
                // Turn off the lamps in the adjacent cells

                for (i, j) in &moves {
                    let (na, nb) = (a + i, b + j);
                    if lighted.contains(&(na, nb)) {
                        lighted.remove(&(na, nb));
                        *rows.entry(na).or_insert(0) -= 1;
                        *cols.entry(nb).or_insert(0) -= 1;
                        *pd.entry(na - nb).or_insert(0) -= 1;
                        *sd.entry(na + nb).or_insert(0) -= 1;
                    }
                }
            } else {
                res.push(0);
            }
        }
        
        // Return the results

        res

    }
}
