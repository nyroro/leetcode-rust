


impl Solution {
    pub fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut sums = vec![0; s.len()];
        let mut plates = Vec::new();

        for (i, c) in s.chars().enumerate() {
            if c == '|' {
                plates.push(i);
            }
            if c == '*' {
                sums[i] += 1;
            }
            if i > 0 {
                sums[i] += sums[i - 1];
            }
        }

        let mut ret = Vec::new();

        if plates.is_empty() {
            return vec![0; queries.len()];
        }

        for query in &queries {
            let (mut left, mut right) = (0, plates.len() - 1);
            let (s, t) = (query[0] as usize, query[1] as usize);

            while left < right {
                let mid = (left + right) / 2;
                if plates[mid] < s {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }

            let s = left;

            left = 0;
            right = plates.len() - 1;

            while left < right {
                let mid = (left + right + 1) / 2;
                if plates[mid] > t {
                    right = mid - 1;
                } else {
                    left = mid;
                }
            }

            let t = right;

            if s < plates.len() && t < plates.len() && plates[t] > t && s < t {
                ret.push(sums[plates[t]] - sums[plates[s]]);
            } else {
                ret.push(0);
            }
        }

        ret

    }
}
