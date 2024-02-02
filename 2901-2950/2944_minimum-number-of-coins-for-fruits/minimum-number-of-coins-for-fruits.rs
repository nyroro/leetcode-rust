
// Define the Solution struct



impl Solution {
    // Define the minimum_coins function

    pub fn minimum_coins(prices: Vec<i32>) -> i32 {
        // Create a vector to store the minimum coins needed for each fruit

        let mut dp = vec![std::i32::MAX; prices.len() + 1];
        
        // Set the base case for the last fruit

        dp[prices.len()] = 0;
        
        // Iterate through the prices in reverse order

        for i in (0..prices.len()).rev() {
            // Initialize the variable for the minimum coins needed to buy the current fruit

            let mut min_coins = std::i32::MAX;
            
            // Calculate the minimum coins needed to buy the current fruit

            for j in (i..(2 * i + 2).min(prices.len())).rev() {
                min_coins = min_coins.min(prices[i] + dp[j + 1]);
            }
            
            // Store the minimum coins needed in the dp vector

            dp[i] = min_coins;
        }
        
        // Return the minimum coins needed to acquire all the fruits

        dp[0]
    }
}
