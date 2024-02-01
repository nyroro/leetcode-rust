
impl Solution {
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        let mut croak_count = vec![0; 5];
        let mut frogs = 0;
        let croak_order = ['c', 'r', 'o', 'a', 'k'];
        
        for c in croak_of_frogs.chars() {
            match c {
                'c' => croak_count[0] += 1,
                'r' => {
                    if croak_count[0] > croak_count[1] {
                        return -1;
                    }
                    croak_count[1] += 1;
                },
                'o' => {
                    if croak_count[1] > croak_count[2] {
                        return -1;
                    }
                    croak_count[2] += 1;
                },
                'a' => {
                    if croak_count[2] > croak_count[3] {
                        return -1;
                    }
                    croak_count[3] += 1;
                },
                'k' => {
                    if croak_count[3] > croak_count[4] {
                        return -1;
                    }
                    croak_count[4] += 1;
                    for i in 0..5 {
                        croak_count[i] -= 1;
                    }
                    frogs = frogs.max(croak_count[0]);
                },
                _ => return -1,
            }
        }
        
        if croak_count.iter().all(|&x| x == 0) {
            frogs

        } else {
            -1

        }
    }
}
