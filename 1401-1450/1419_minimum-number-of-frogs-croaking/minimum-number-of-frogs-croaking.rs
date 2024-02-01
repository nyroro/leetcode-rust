
impl Solution {
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        let mut counts = vec![0; 5];
        let croak_order = ['c', 'r', 'o', 'a', 'k'];
        let mut frogs = 0;

        for c in croak_of_frogs.chars() {
            match c {
                'c' => counts[0] += 1,
                'r' => {
                    if counts[0] <= counts[1] {
                        return -1;
                    }
                    counts[1] += 1;
                },
                'o' => {
                    if counts[1] <= counts[2] {
                        return -1;
                    }
                    counts[2] += 1;
                },
                'a' => {
                    if counts[2] <= counts[3] {
                        return -1;
                    }
                    counts[3] += 1;
                },
                'k' => {
                    if counts[3] <= counts[4] {
                        return -1;
                    }
                    counts[4] += 1;
                    for i in 0..5 {
                        counts[i] -= 1;
                    }
                    frogs = frogs.max(counts[0]);
                },
                _ => return -1,
            }
        }

        if counts.iter().all(|&x| x == 0) {
            frogs + 1

        } else {
            -1

        }
    }
}
