
impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new(); // Step 1: Create a new array to store the final prices

        for i in 0..prices.len() { // Step 2: Iterate through the given prices array

            let mut discount = 0; // Initialize the discount as 0

            for j in (i + 1)..prices.len() { // Step 3: Find the first smaller price index

                if prices[j] <= prices[i] { // Compare the prices

                    discount = prices[j]; // Update the discount

                    break; // Exit the loop once a smaller price is found

                }
            }
            result.push(prices[i] - discount); // Step 4: Calculate the final price and store it in the new array

        }
        result // Step 5: Return the new array as the final result

    }
}
