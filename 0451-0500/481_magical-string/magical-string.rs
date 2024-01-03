
impl Solution {
    pub fn magical_string(n: i32) -> i32 {
        // Create a vector to store the magical string

        let mut magic_string = vec![1, 2, 2];
        let mut index = 2;
        let mut num_of_ones = 1;

        // Generate the magical string

        while magic_string.len() < n as usize {
            let next_num = 3 - magic_string[magic_string.len() - 1];
            for _ in 0..magic_string[index] {
                magic_string.push(next_num);
                if next_num == 1 && magic_string.len() <= n as usize {
                    num_of_ones += 1;
                }
            }
            index += 1;
        }

        num_of_ones

    }
}
