
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs()
}

impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();

        let mut pairs = Vec::new();
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                pairs.push((gcd(nums[i], nums[j]), i, j));
            }
        }

        pairs.sort_by(|a, b| b.0.cmp(&a.0));

        let mut max_score = 0;
        for fixed_pair in pairs.iter() {
            let mut used = vec![fixed_pair.1, fixed_pair.2];
            let mut gcds = vec![fixed_pair.0];

            for _ in 0..(nums.len() / 2 - 1) {
                for pair in pairs.iter() {
                    if !used.contains(&pair.1) && !used.contains(&pair.2) {
                        used.push(pair.1);
                        used.push(pair.2);
                        gcds.push(pair.0);
                    }
                }
            }

            gcds.sort();
            let mut cur_score = 0;
            for (j, &gcd) in gcds.iter().enumerate() {
                cur_score += (j as i32 + 1) * gcd;
            }
            max_score = max_score.max(cur_score);
        }
        max_score

    }
}
