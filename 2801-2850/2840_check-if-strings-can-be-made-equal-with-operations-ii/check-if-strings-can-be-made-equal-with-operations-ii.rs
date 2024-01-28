


impl Solution {
    pub fn check_strings(s1: String, s2: String) -> bool {
        let mut even1: Vec<char> = Vec::new();
        let mut even2: Vec<char> = Vec::new();
        let mut odd1: Vec<char> = Vec::new();
        let mut odd2: Vec<char> = Vec::new();

        // Populate even and odd vectors for s1 and s2

        for (i, ch) in s1.chars().enumerate() {
            if i % 2 == 0 {
                even1.push(ch);
            } else {
                odd1.push(ch);
            }
        }

        for (i, ch) in s2.chars().enumerate() {
            if i % 2 == 0 {
                even2.push(ch);
            } else {
                odd2.push(ch);
            }
        }

        // Sort the even and odd vectors

        even1.sort();
        even2.sort();
        odd1.sort();
        odd2.sort();

        // Compare the sorted vectors

        even1 == even2 && odd1 == odd2

    }
}
