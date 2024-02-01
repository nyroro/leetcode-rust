
struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> UnionFind {
        UnionFind { parent: (0..size).collect() }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x

    }
}



impl Solution {
    pub fn friend_requests(n: i32, restrictions: Vec<Vec<i32>>, requests: Vec<Vec<i32>>) -> Vec<bool> {
        let mut pre: Vec<usize> = (0..n as usize).collect();
        let mut ret: Vec<bool> = Vec::new();
        let mut uf = UnionFind::new(n as usize);

        for request in requests {
            let u = request[0] as usize;
            let v = request[1] as usize;

            let tu = uf.find(u);
            let tv = uf.find(v);

            let mut t_pre = pre.clone();
            t_pre[tu] = tv;

            for restriction in &restrictions {
                let x = restriction[0] as usize;
                let y = restriction[1] as usize;

                if uf.find(x) == uf.find(y) {
                    ret.push(false);
                    break;
                }
            }

            if ret.len() < ret.len() + 1 {
                pre = t_pre;
                ret.push(true);
            }
        }

        ret

    }
}
