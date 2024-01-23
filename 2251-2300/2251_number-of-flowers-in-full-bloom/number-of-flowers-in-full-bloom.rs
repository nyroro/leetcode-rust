
struct Flower {
    start: i32,
    end: i32,
}

impl Solution {
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        let mut arr: Vec<(i32, i32)> = Vec::new();
        for flower in &flowers {
            arr.push((flower[0], -1));
            arr.push((flower[1], people.len() as i32 + 1));
        }
        for (i, &p) in people.iter().enumerate() {
            arr.push((p, i as i32));
        }
        let mut ret: Vec<i32> = vec![0; people.len()];

        arr.sort();

        let mut now = 0;
        for &(x, y) in &arr {
            if y < 0 {
                now += 1;
            } else if y > people.len() as i32 {
                now -= 1;
            } else {
                ret[y as usize] = now;
            }
        }
        ret

    }
}
