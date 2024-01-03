
impl Solution {
    pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
        // Create a variable to store the total tax

        let mut tax: f64 = 0.0;

        // Iterate through each tax bracket

        for i in 0..brackets.len() {
            // Calculate the amount of money in the current tax bracket

            let current_income = if i == 0 {
                income.min(brackets[i][0])
            } else {
                (income - brackets[i - 1][0]).max(0).min(brackets[i][0] - brackets[i - 1][0])
            };

            // Calculate the tax for the current tax bracket and add it to the total tax

            tax += current_income as f64 * brackets[i][1] as f64 / 100.0;
        }

        // Return the total tax

        tax

    }
}
