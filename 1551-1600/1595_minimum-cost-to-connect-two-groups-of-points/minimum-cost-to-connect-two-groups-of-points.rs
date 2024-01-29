
use std::collections::HashSet;

impl Solution {
    pub fn connect_two_groups(cost: Vec<Vec<i32>>) -> i32 {
        let size1 = cost.len();
        let size2 = cost[0].len();

        fn int_to_set(x: usize, size: usize) -> HashSet<usize> {
            let mut set = HashSet::new();
            for i in 0..size {
                if (x >> i) & 1 == 1 {
                    set.insert(i);
                }
            }
            set

        }

        fn set_to_int(arr: &HashSet<usize>) -> usize {
            arr.iter().fold(0, |acc, &x| acc | (1 << x))
        }

        fn set_size(s: &HashSet<usize>) -> usize {
            s.len()
        }

        let full_set2_int = (1 << size2) - 1;
        let mut dp = vec![vec![i32::MAX; full_set2_int + 1]; size1 + 1];
        dp[0][0] = 0;

        let min_cost_set1: Vec<i32> = cost.iter().map(|row| *row.iter().min().unwrap()).collect();
        for i in 1..=size1 {
            dp[i][0] = dp[i - 1][0] + min_cost_set1[i - 1];
        }

        let mut min_cost_set2 = vec![i32::MAX; size2];
        for j in 0..size2 {
            for i in 0..size1 {
                min_cost_set2[j] = min_cost_set2[j].min(cost[i][j]);
            }
        }

        for s in 1..=full_set2_int {
            dp[0][s] = 0;
            for j in 0..size2 {
                if (s >> j) & 1 == 1 {
                    dp[0][s] += min_cost_set2[j];
                }
            }
        }

        for i in 1..=size1 {
            for s in 1..=full_set2_int {
                let set2 = int_to_set(s, size2);
                for j in &set2 {
                    let mut reduced_set2_int = s;
                    reduced_set2_int &= !(1 << j);
                    dp[i][s] = dp[i][s].min(dp[i - 1][reduced_set2_int] + cost[i - 1][*j]);
                }
                dp[i][s] = dp[i][s].min(dp[i - 1][s] + min_cost_set1[i - 1]);
            }
        }

        dp[size1][full_set2_int]
    }
}
