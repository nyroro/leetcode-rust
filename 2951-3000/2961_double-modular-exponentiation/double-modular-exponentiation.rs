


impl Solution {
    pub fn get_good_indices(variables: Vec<Vec<i32>>, target: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new(); // Resultant vector to store good indices

        let mut k: i32 = 0; // Counter for keeping track of the index


        for x in &variables {
            let m = x[0];
            let b = x[1];
            let c = x[2];
            let d = x[3];

            let mut n1: i32 = 1;

            for _ in 0..b {
                n1 = (n1 * m) % 10;
            }

            let mut n2: i32 = 1;

            for _ in 0..c {
                n2 = (n2 * n1) % d;
            }

            if n2 == target {
                ans.push(k);
            }

            k += 1;
        }

        ans

    }
}
