


impl Solution {
    pub fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
        let n: usize = max_heights.len();
        let mut prefix = vec![0; n];
        let mut suffix = vec![0; n];
        let mut st: Vec<usize> = Vec::new();

        st.push(0);
        prefix[0] = max_heights[0] as i64;

        for i in 1..n {
            // Calculate prefix sum

            while !st.is_empty() && max_heights[*st.last().unwrap()] > max_heights[i] {
                st.pop();
            }

            if st.is_empty() {
                prefix[i] = (i as i64 + 1) * max_heights[i] as i64;
                st.push(i);
                continue;
            }

            let it = st.last().unwrap();
            prefix[i] = prefix[*it] + (i as i64 - *it as i64) * max_heights[i] as i64;
            st.push(i);
        }

        st.clear();
        st.push(n - 1);
        suffix[n - 1] = max_heights[n - 1] as i64;

        for i in (0..n - 1).rev() {
            // Calculate suffix sum

            while !st.is_empty() && max_heights[*st.last().unwrap()] > max_heights[i] {
                st.pop();
            }

            if st.is_empty() {
                suffix[i] = (n as i64 - i as i64) * max_heights[i] as i64;
                st.push(i);
                continue;
            }

            let it = st.last().unwrap();
            suffix[i] = suffix[*it] + (*it as i64 - i as i64) * max_heights[i] as i64;
            st.push(i);
        }

        let mut max_sum = 0;
        for i in 0..n {
            max_sum = max_sum.max(prefix[i] + suffix[i] - max_heights[i] as i64);
        }

        max_sum

    }
}
