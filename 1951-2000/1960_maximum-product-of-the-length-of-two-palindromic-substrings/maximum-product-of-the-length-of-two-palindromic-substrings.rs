


impl Solution {
    pub fn max_product(s: String) -> i64 {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();

        let mut hlen = vec![0; n];
        let (mut center, mut right) = (0, 0);
        for i in 0..n {
            if i < right {
                hlen[i] = std::cmp::min(right - i, hlen[2 * center - i]);
            }
            while i >= 1 + hlen[i] && i + 1 + hlen[i] < n && s[i - 1 - hlen[i]] == s[i + 1 + hlen[i]] {
                hlen[i] += 1;
            }
            if right < i + hlen[i] {
                center = i;
                right = i + hlen[i];
            }
        }

        let mut prefix = vec![0; n];
        let mut suffix = vec![0; n];
        for i in 0..n {
            prefix[i + hlen[i]] = std::cmp::max(prefix[i + hlen[i]], 2 * hlen[i] as i64 + 1);
            suffix[i.saturating_sub(hlen[i])] = std::cmp::max(suffix[i.saturating_sub(hlen[i])], 2 * hlen[i] as i64 + 1);
        }

        for i in 1..n {
            prefix[n - i - 1] = std::cmp::max(prefix[n - i - 1], prefix[n - i] - 2);
            suffix[i] = std::cmp::max(suffix[i], suffix[i - 1] - 2);
        }

        for i in 1..n {
            prefix[i] = std::cmp::max(prefix[i - 1], prefix[i]);
            suffix[n - i - 1] = std::cmp::max(suffix[n - i - 1], suffix[n - i]);
        }

        let mut max_product = 0;
        for i in 1..n {
            max_product = std::cmp::max(max_product, prefix[i - 1] * suffix[i]);
        }

        max_product

    }
}
