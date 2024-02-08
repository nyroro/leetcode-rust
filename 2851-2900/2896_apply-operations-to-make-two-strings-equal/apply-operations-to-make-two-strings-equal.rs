
// Define a struct to represent the Solution



impl Solution {
    // Define a helper function to perform the recursive calculation

    fn rec(ind1: usize, prev: usize, co: usize, s1: &Vec<char>, s2: &Vec<char>, n: usize, x: i32, dp: &mut Vec<Vec<Vec<i32>>>) -> i32 {
        // Base case: if ind1 reaches the end of the string

        if ind1 == n {
            return (co / 2) as i32 * x;
        }
        
        // If the result is already calculated, return it

        if dp[ind1][prev][co] != -1 {
            return dp[ind1][prev][co];
        }
        
        let mut ans = std::i32::MAX;
        
        // Check if the characters at ind1 in s1 and s2 are equal

        if s1[ind1] == s2[ind1] {
            if prev == 1 {
                ans = 1 + ans.min(Solution::rec(ind1 + 1, 1, co, s1, s2, n, x, dp));
            }
            ans = ans.min(Solution::rec(ind1 + 1, 0, co, s1, s2, n, x, dp));
        } else {
            if prev == 0 {
                ans = ans.min(Solution::rec(ind1 + 1, 1, co + 1, s1, s2, n, x, dp));
            } else if prev == 2 {
                ans = ans.min(Solution::rec(ind1 + 1, 1, co + 1, s1, s2, n, x, dp));
            } else {
                ans = 1 + ans.min(Solution::rec(ind1 + 1, 0, co - 1, s1, s2, n, x, dp));
                ans = ans.min(Solution::rec(ind1 + 1, 1, co + 1, s1, s2, n, x, dp));
            }
        }
        
        dp[ind1][prev][co] = ans;
        ans

    }

    // Implement the min_operations function

    pub fn min_operations(s1: String, s2: String, x: i32) -> i32 {
        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();
        let n = s1_chars.len();
        let mut dp = vec![vec![vec![-1; n + 1]; 3]; n + 1];
        let mut c = 0;

        for i in 0..n {
            if s1_chars[i] != s2_chars[i] {
                c += 1;
            }
        }

        if c % 2 != 0 {
            return -1;
        } else {
            Solution::rec(0, 2, 0, &s1_chars, &s2_chars, n, x, &mut dp)
        }
    }
}
