


impl Solution {
    pub fn is_it_possible(word1: String, word2: String) -> bool {
        let mut cnt1 = vec![0; 26];
        let mut cnt2 = vec![0; 26];

        for ch in word1.bytes() {
            cnt1[(ch - b'a') as usize] += 1;
        }

        for ch in word2.bytes() {
            cnt2[(ch - b'a') as usize] += 1;
        }

        for i in 0..26 {
            for j in 0..26 {
                if cnt1[i] != 0 && cnt2[j] != 0 {
                    // Swap

                    cnt1[j] += 1;
                    cnt2[j] -= 1;
                    cnt1[i] -= 1;
                    cnt2[i] += 1;
                    // Check Validity

                    if Solution::is_valid(&cnt1, &cnt2) {
                        return true;
                    }
                    // Restore

                    cnt1[j] -= 1;
                    cnt2[j] += 1;
                    cnt1[i] += 1;
                    cnt2[i] -= 1;
                }
            }
        }
        false

    }

    fn is_valid(cnt1: &Vec<i32>, cnt2: &Vec<i32>) -> bool {
        let mut d1 = 0;
        let mut d2 = 0;
        for i in 0..26 {
            if cnt1[i] != 0 {
                d1 += 1;
            }
            if cnt2[i] != 0 {
                d2 += 1;
            }
        }
        d1 == d2

    }
}
