
// Define a struct to represent the state of the algorithm

struct MinInteger {
    num: Vec<u8>,  // Convert the input string to a vector of u8 digits

    k: i32,  // Number of swaps allowed

}

impl MinInteger {
    // Implement a function to find the minimum integer

    pub fn min_integer(num: String, k: i32) -> String {
        // Convert the input string to a vector of u8 digits

        let num: Vec<u8> = num.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();

        // Create a vector to store the number of digits to the right of each index

        let mut arr: Vec<i32> = vec![0; num.len() + 1];

        // Create a vector to track whether a digit has been used in a swap

        let mut used: Vec<bool> = vec![false; num.len()];

        // Create a vector of vectors to store the indices of each digit

        let mut table: Vec<Vec<usize>> = vec![Vec::new(); 10];

        // Populate the table with the indices of each digit

        for (i, &t) in num.iter().enumerate() {
            table[t as usize].push(i);
            let mut x = i + 1;
            while x <= num.len() {
                arr[x] += 1;
                x += x & (!x + 1);
            }
        }

        // Create a vector to store the resulting minimum integer

        let mut ret: Vec<u8> = Vec::new();

        // Initialize the right boundary

        let mut r = num.len();

        // Iterate over each digit

        for i in 0..10 {
            // Iterate over the indices of the current digit

            for &index in table[i].iter() {
                let mut x = index;
                if index > r {
                    break;
                }
                let mut s = 0;
                while x > 0 {
                    s += arr[x];
                    x -= x & (!x + 1);
                }
                if k >= s {
                    ret.push(i as u8);
                    used[index] = true;
                    k -= s;
                    x = index + 1;
                    while x <= num.len() {
                        arr[x] -= 1;
                        x += x & (!x + 1);
                    }
                } else {
                    r = index;
                    break;
                }
            }
        }

        // Append the remaining unused digits to the result

        for (i, &t) in num.iter().enumerate() {
            if !used[i] {
                ret.push(t);
                let mut index = ret.len() - 1;
                while k > 0 && index > 0 && ret[index] < ret[index - 1] {
                    ret.swap(index, index - 1);
                    index -= 1;
                    k -= 1;
                }
            }
        }

        // Convert the resulting vector of u8 digits to a string

        ret.iter().map(|&c| char::from_digit(c as u32, 10).unwrap()).collect()
    }
}
