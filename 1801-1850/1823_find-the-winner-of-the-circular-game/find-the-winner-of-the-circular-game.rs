
impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut friends = Vec::new();
        let mut pointer = 0;

        for i in 1..=n {
            friends.push(i);
        }

        while friends.len() > 1 {
            pointer = (pointer + k as usize - 1) % friends.len();
            friends.remove(pointer);
        }

        friends[0]
    }
}
