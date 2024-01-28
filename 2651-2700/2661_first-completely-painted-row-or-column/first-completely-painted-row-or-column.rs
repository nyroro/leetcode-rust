
use std::collections::HashMap;

impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let mut m: HashMap<i32, usize> = HashMap::new();
        let (r, c) = (mat.len(), mat[0].len());

        for (i, &n) in arr.iter().enumerate() {
            m.insert(n, i);
        }

        let mut ans = 1000000000;

        for i in 0..r {
            let mut curminind = 0;
            for j in 0..c {
                curminind = curminind.max(*m.get(&mat[i][j]).unwrap());
            }
            ans = ans.min(curminind);
        }

        for i in 0..c {
            let mut curminind = 0;
            for j in 0..r {
                curminind = curminind.max(*m.get(&mat[j][i]).unwrap());
            }
            ans = ans.min(curminind);
        }

        ans as i32

    }
}
