


impl Solution {
    pub fn number_game(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort(); // Sort the input array


        let mut arr: Vec<i32> = Vec::new();

        while !nums.is_empty() {
            let alice_move = nums.remove(0); // Alice removes the minimum element

            let bob_move = nums.remove(0); // Bob removes the minimum element

            arr.push(bob_move); // Bob appends the removed element

            arr.push(alice_move); // Alice appends the removed element

        }

        arr

    }
}
