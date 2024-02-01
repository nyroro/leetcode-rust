


impl Solution {
    pub fn friend_requests(n: i32, restrictions: Vec<Vec<i32>>, requests: Vec<Vec<i32>>) -> Vec<bool> {
        let mut pre: Vec<usize> = (0..n as usize).collect();
        let mut ret: Vec<bool> = Vec::new();

        fn find(pre: &mut Vec<usize>, mut x: usize) -> usize {
            while pre[x] != x {
                pre[x] = pre[pre[x]];
                x = pre[x];
            }
            x

        }

        for request in requests {
            let u = request[0] as usize;
            let v = request[1] as usize;

            let tu = find(&mut pre, u);
            let tv = find(&mut pre, v);

            let mut t_pre = pre.clone();
            t_pre[tu] = tv;

            let mut is_restricted = false;
            for restriction in &restrictions {
                let x = restriction[0] as usize;
                let y = restriction[1] as usize;

                if find(&mut t_pre, x) == find(&mut t_pre, y) {
                    is_restricted = true;
                    break;
                }
            }

            if is_restricted {
                ret.push(false);
            } else {
                pre = t_pre;
                ret.push(true);
            }
        }

        ret

    }
}
