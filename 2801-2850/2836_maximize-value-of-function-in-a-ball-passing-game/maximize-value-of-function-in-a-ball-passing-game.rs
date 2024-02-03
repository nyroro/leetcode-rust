
// Define the Solution struct



impl Solution {
    // Implement the get_max_function_value method

    pub fn get_max_function_value(receiver: Vec<i32>, k: i64) -> i64 {
        let n = receiver.len() as i64; // Convert n to i64


        // Initialize parent and path_sum arrays

        let mut parent = vec![vec![0; 35]; n as usize];
        let mut path_sum = vec![vec![0; 35]; n as usize];
        
        // Populate parent and path_sum arrays

        for start in 0..n {
            parent[start as usize][0] = receiver[start as usize] as i64; // Convert receiver to i64

            path_sum[start as usize][0] = receiver[start as usize] as i64; // Convert receiver to i64

        }

        for power in 1..35 {
            for start in 0..n {
                parent[start as usize][power as usize] = parent[parent[start as usize][power as usize - 1] as usize][power as usize - 1];
                path_sum[start as usize][power as usize] = path_sum[start as usize][power as usize - 1] + path_sum[parent[start as usize][power as usize - 1] as usize][power as usize - 1];
            }
        }

        let mut res = 0;

        // Calculate the maximum function value

        for start in 0..n {
            let mut i = start;
            let mut running_sum = 0;

            for power in 0..35 {
                if k & (1 << power) != 0 {
                    running_sum += path_sum[i as usize][power as usize];
                    i = parent[i as usize][power as usize];
                }
            }

            res = res.max(start + running_sum);
        }

        res

    }
}
