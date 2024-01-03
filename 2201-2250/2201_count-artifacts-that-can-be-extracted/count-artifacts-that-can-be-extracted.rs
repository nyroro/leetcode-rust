
impl Solution {
    pub fn dig_artifacts(n: i32, artifacts: Vec<Vec<i32>>, dig: Vec<Vec<i32>>) -> i32 {
        let mut grid = vec![vec![0; n as usize]; n as usize];
        let mut count = 0;

        // Set artifact areas to 1

        for artifact in &artifacts {
            let r1 = artifact[0] as usize;
            let c1 = artifact[1] as usize;
            let r2 = artifact[2] as usize;
            let c2 = artifact[3] as usize;
            for i in r1..=r2 {
                for j in c1..=c2 {
                    grid[i][j] = 1;
                }
            }
        }

        // Excavate cells

        for d in dig {
            let r = d[0] as usize;
            let c = d[1] as usize;
            grid[r][c] = 2;
        }

        // Check artifacts

        for artifact in &artifacts {
            let r1 = artifact[0] as usize;
            let c1 = artifact[1] as usize;
            let r2 = artifact[2] as usize;
            let c2 = artifact[3] as usize;
            let mut uncovered = true;
            for i in r1..=r2 {
                for j in c1..=c2 {
                    if grid[i][j] != 2 {
                        uncovered = false;
                        break;
                    }
                }
                if !uncovered {
                    break;
                }
            }
            if uncovered {
                count += 1;
            }
        }

        count

    }
}
