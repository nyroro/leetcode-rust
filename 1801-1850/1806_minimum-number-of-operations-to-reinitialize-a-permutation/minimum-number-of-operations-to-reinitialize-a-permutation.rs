
impl Solution {
    pub fn reinitialize_permutation(n: i32) -> i32 {
        let mut perm: Vec<i32> = (0..n).collect();
        let mut arr: Vec<i32> = vec![0; n as usize];
        let mut operations = 0;

        loop {
            for i in 0..n {
                if i % 2 == 0 {
                    arr[i as usize] = perm[(i / 2) as usize];
                } else {
                    arr[i as usize] = perm[(n / 2 + (i - 1) / 2) as usize];
                }
            }

            perm = arr.clone();
            operations += 1;

            if perm.iter().enumerate().all(|(i, &val)| i as i32 == val) {
                break;
            }
        }

        operations

    }
}
