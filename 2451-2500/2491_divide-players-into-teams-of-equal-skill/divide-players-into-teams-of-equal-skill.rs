
impl Solution {
    pub fn divide_players(mut skill: Vec<i32>) -> i64 {
        // Sort the skill array

        skill.sort();

        // Initialize the target sum

        let n = skill.len();
        let target = skill[0] + skill[n - 1];

        // Initialize the total chemistry sum

        let mut ans = 0;

        // Iterate through the skill array to form teams and calculate chemistry sum

        for i in 0..n / 2 {
            if skill[i] + skill[n - 1 - i] != target {
                return -1;
            }
            ans += (skill[i] * skill[n - 1 - i]) as i64;
        }

        // Return the total chemistry sum

        ans

    }
}
