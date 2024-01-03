
impl Solution {
    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        // Create a set to store the valid gene mutations

        let bank_set: std::collections::HashSet<String> = bank.into_iter().collect();
        
        // Create a queue to perform BFS

        let mut queue: std::collections::VecDeque<(String, i32)> = std::collections::VecDeque::new();
        
        // Add the start gene to the queue

        queue.push_back((start_gene.clone(), 0));
        
        // Create a set to store the visited genes

        let mut visited: std::collections::HashSet<String> = std::collections::HashSet::new();
        
        // Perform BFS

        while let Some((gene, steps)) = queue.pop_front() {
            // Check if the current gene is the end gene

            if gene == end_gene {
                return steps;
            }
            
            // Iterate through each character in the gene

            for i in 0..gene.len() {
                // Iterate through each possible mutation

                for c in &['A', 'C', 'G', 'T'] {
                    // Create a new gene by replacing the character at index i

                    let new_gene = gene.chars().enumerate().map(|(j, x)| if j == i { *c } else { x }).collect::<String>();
                    
                    // Check if the new gene is a valid mutation and has not been visited

                    if bank_set.contains(&new_gene) && !visited.contains(&new_gene) {
                        // Add the new gene to the queue and mark it as visited

                        queue.push_back((new_gene.clone(), steps + 1));
                        visited.insert(new_gene);
                    }
                }
            }
        }
        
        // If no valid mutation is found, return -1

        -1

    }
}
