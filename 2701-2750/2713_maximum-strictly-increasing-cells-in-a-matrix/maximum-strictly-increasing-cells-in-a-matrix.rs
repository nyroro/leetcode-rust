
use std::collections::{HashMap, BTreeSet};
use std::cmp;

impl Solution {
    pub fn max_increasing_cells(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut r = vec![0; m];
        let mut c = vec![0; n];
        let mut vmap: HashMap<i32, Vec<(usize, usize)>> = HashMap::new();
        let mut s: BTreeSet<i32> = BTreeSet::new();

        for i in 0..m {
            for j in 0..n {
                let val = -mat[i][j];
                if !vmap.contains_key(&val) {
                    vmap.insert(val, Vec::new());
                }
                vmap.get_mut(&val).unwrap().push((i, j));
                s.insert(val);
            }
        }

        let mut temp = vec![vec![0; n]; m];
        for x in s.iter() {
            if let Some(cells) = vmap.get(x) {
                for v in cells {
                    temp[v.0][v.1] = cmp::max(r[v.0], c[v.1]) + 1;
                }
                for v in cells {
                    r[v.0] = cmp::max(r[v.0], temp[v.0][v.1]);
                    c[v.1] = cmp::max(c[v.1], temp[v.0][v.1]);
                }
            }
        }

        cmp::max(*r.iter().max().unwrap(), *c.iter().max().unwrap())
    }
}
