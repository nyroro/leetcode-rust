
impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let mut graph = vec![vec![]; n as usize];
        let mut colors = vec![0; n as usize];

        for dislike in dislikes {
            let a = dislike[0] as usize - 1;
            let b = dislike[1] as usize - 1;
            graph[a].push(b);
            graph[b].push(a);
        }

        fn dfs(node: usize, color: i32, graph: &Vec<Vec<usize>>, colors: &mut Vec<i32>) -> bool {
            colors[node] = color;
            for &neighbor in &graph[node] {
                if colors[neighbor] == color {
                    return false;
                }
                if colors[neighbor] == 0 && !dfs(neighbor, -color, graph, colors) {
                    return false;
                }
            }
            true

        }

        for i in 0..n as usize {
            if colors[i] == 0 && !dfs(i, 1, &graph, &mut colors) {
                return false;
            }
        }

        true

    }
}
