
impl Solution {
    pub fn find_latest_step(arr: Vec<i32>, m: i32) -> i32 {
        let mut ret = -1;
        let mut s: std::collections::HashSet<usize> = std::collections::HashSet::new();
        let n = arr.len();
        let mut seg: Vec<Option<(usize, i32)>> = vec![None; n];

        fn merge(seg: &mut Vec<Option<(usize, i32)>>, s: &mut std::collections::HashSet<usize>, x: usize, y: usize, m: i32) {
            let mut nx = x;
            while seg[nx].unwrap().0 != nx {
                nx = seg[nx].unwrap().0;
            }
            
            let mut ny = y;
            while seg[ny].unwrap().0 != ny {
                ny = seg[ny].unwrap().0;
            }

            if seg[nx].unwrap().1 == m {
                s.remove(&nx);
            }
            if seg[ny].unwrap().1 == m {
                s.remove(&ny);
            }
            let val = (nx, seg[nx].unwrap().1 + seg[ny].unwrap().1);
            seg[nx] = Some(val);
            seg[ny] = Some(val);
            if val.1 == m {
                s.insert(nx);
            }
        }

        for (i, &t) in arr.iter().enumerate() {
            let t = (t - 1) as usize;

            if seg[t].is_none() {
                seg[t] = Some((t, 1));
                if m == 1 {
                    s.insert(t);
                }
                if t >= 1 && seg[t - 1].is_some() {
                    merge(&mut seg, &mut s, t - 1, t, m);
                }
                if t + 1 < n && seg[t + 1].is_some() {
                    merge(&mut seg, &mut s, t, t + 1, m);
                }
            }
            if !s.is_empty() {
                ret = (i + 1) as i32;
            }
        }

        ret

    }
}
