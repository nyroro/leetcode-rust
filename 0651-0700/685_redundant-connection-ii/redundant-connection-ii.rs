
impl Solution {
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut parent = vec![0; n + 1];
        let mut first = vec![0; n + 1];
        let mut second = vec![0; n + 1];
        let mut candidates = vec![];

        for edge in &edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            if parent[v] != 0 {
                first = vec![u as i32, v as i32];
                second = edge.clone();
            } else {
                parent[v] = u as i32;
            }
        }

        for i in 1..=n {
            parent[i] = i as i32;
        }

        for edge in &edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            if edge == second {
                continue;
            }
            let pu = Self::find(&parent, u);
            let pv = Self::find(&parent, v);
            if pu == pv {
                candidates = first.clone();
            } else {
                parent[pv] = pu;
            }
        }

        if candidates.is_empty() {
            second

        } else {
            candidates

        }
    }

    fn find(parent: &Vec<i32>, mut x: usize) -> i32 {
        while x as i32 != parent[x] {
            x = parent[x] as usize;
        }
        x as i32

    }
}
