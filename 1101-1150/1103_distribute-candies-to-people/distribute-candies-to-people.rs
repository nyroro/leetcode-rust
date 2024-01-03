
impl Solution {
    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        let mut ans = vec![0; num_people as usize];
        let mut candies_left = candies;
        let mut give_candies = 1;
        let mut idx = 0;

        while candies_left > 0 {
            ans[idx as usize] += std::cmp::min(give_candies, candies_left);
            candies_left -= give_candies;
            give_candies += 1;
            idx = (idx + 1) % num_people;
        }

        ans

    }
}
